/* automatically generated by rust-bindgen 0.59.1 */

pub type integer_t = ::std::os::raw::c_int;
pub type cpu_type_t = integer_t;
pub type cpu_subtype_t = integer_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mach_header {
	pub magic: u32,
	pub cputype: cpu_type_t,
	pub cpusubtype: cpu_subtype_t,
	pub filetype: u32,
	pub ncmds: u32,
	pub sizeofcmds: u32,
	pub flags: u32,
}
#[test]
fn bindgen_test_layout_mach_header() {
	assert_eq!(
		::std::mem::size_of::<mach_header>(),
		28usize,
		concat!("Size of: ", stringify!(mach_header))
	);
	assert_eq!(
		::std::mem::align_of::<mach_header>(),
		4usize,
		concat!("Alignment of ", stringify!(mach_header))
	);
	assert_eq!(
		unsafe { &(*(::std::ptr::null::<mach_header>())).magic as *const _ as usize },
		0usize,
		concat!(
			"Offset of field: ",
			stringify!(mach_header),
			"::",
			stringify!(magic)
		)
	);
	assert_eq!(
		unsafe { &(*(::std::ptr::null::<mach_header>())).cputype as *const _ as usize },
		4usize,
		concat!(
			"Offset of field: ",
			stringify!(mach_header),
			"::",
			stringify!(cputype)
		)
	);
	assert_eq!(
		unsafe { &(*(::std::ptr::null::<mach_header>())).cpusubtype as *const _ as usize },
		8usize,
		concat!(
			"Offset of field: ",
			stringify!(mach_header),
			"::",
			stringify!(cpusubtype)
		)
	);
	assert_eq!(
		unsafe { &(*(::std::ptr::null::<mach_header>())).filetype as *const _ as usize },
		12usize,
		concat!(
			"Offset of field: ",
			stringify!(mach_header),
			"::",
			stringify!(filetype)
		)
	);
	assert_eq!(
		unsafe { &(*(::std::ptr::null::<mach_header>())).ncmds as *const _ as usize },
		16usize,
		concat!(
			"Offset of field: ",
			stringify!(mach_header),
			"::",
			stringify!(ncmds)
		)
	);
	assert_eq!(
		unsafe { &(*(::std::ptr::null::<mach_header>())).sizeofcmds as *const _ as usize },
		20usize,
		concat!(
			"Offset of field: ",
			stringify!(mach_header),
			"::",
			stringify!(sizeofcmds)
		)
	);
	assert_eq!(
		unsafe { &(*(::std::ptr::null::<mach_header>())).flags as *const _ as usize },
		24usize,
		concat!(
			"Offset of field: ",
			stringify!(mach_header),
			"::",
			stringify!(flags)
		)
	);
}
pub type os_function_t =
	::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>;
pub type os_block_t = *mut ::std::os::raw::c_void;
extern "C" {
	pub fn os_release(object: *mut ::std::os::raw::c_void);
}
extern "C" {
	pub static mut __dso_handle: mach_header;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct os_log_s {
	_unused: [u8; 0],
}
pub type os_log_t = *mut os_log_s;
pub const os_log_type_t_OS_LOG_TYPE_DEFAULT: os_log_type_t = 0;
pub const os_log_type_t_OS_LOG_TYPE_INFO: os_log_type_t = 1;
pub const os_log_type_t_OS_LOG_TYPE_DEBUG: os_log_type_t = 2;
pub const os_log_type_t_OS_LOG_TYPE_ERROR: os_log_type_t = 16;
pub const os_log_type_t_OS_LOG_TYPE_FAULT: os_log_type_t = 17;
pub type os_log_type_t = u8;
extern "C" {
	pub fn os_log_create(
		subsystem: *const ::std::os::raw::c_char,
		category: *const ::std::os::raw::c_char,
	) -> os_log_t;
}
extern "C" {
	pub fn os_log_type_enabled(oslog: os_log_t, type_: os_log_type_t) -> bool;
}
extern "C" {
	pub fn os_log_is_enabled(log: os_log_t) -> bool;
}
extern "C" {
	pub fn os_log_is_debug_enabled(log: os_log_t) -> bool;
}
pub const os_activity_flag_t_OS_ACTIVITY_FLAG_DEFAULT: os_activity_flag_t = 0;
pub const os_activity_flag_t_OS_ACTIVITY_FLAG_DETACHED: os_activity_flag_t = 1;
pub const os_activity_flag_t_OS_ACTIVITY_FLAG_IF_NONE_PRESENT: os_activity_flag_t = 2;
pub type os_activity_flag_t = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct os_activity_s {
	_unused: [u8; 0],
}
pub type os_activity_t = *mut os_activity_s;
extern "C" {
	pub static mut _os_activity_none: os_activity_s;
}
extern "C" {
	pub static mut _os_activity_current: os_activity_s;
}
pub type os_activity_id_t = u64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct os_activity_scope_state_s {
	pub opaque: [u64; 2usize],
}
#[test]
fn bindgen_test_layout_os_activity_scope_state_s() {
	assert_eq!(
		::std::mem::size_of::<os_activity_scope_state_s>(),
		16usize,
		concat!("Size of: ", stringify!(os_activity_scope_state_s))
	);
	assert_eq!(
		::std::mem::align_of::<os_activity_scope_state_s>(),
		8usize,
		concat!("Alignment of ", stringify!(os_activity_scope_state_s))
	);
	assert_eq!(
		unsafe {
			&(*(::std::ptr::null::<os_activity_scope_state_s>())).opaque as *const _ as usize
		},
		0usize,
		concat!(
			"Offset of field: ",
			stringify!(os_activity_scope_state_s),
			"::",
			stringify!(opaque)
		)
	);
}
pub type os_activity_scope_state_t = *mut os_activity_scope_state_s;
extern "C" {
	pub fn _os_activity_create(
		dso: *mut ::std::os::raw::c_void,
		description: *const ::std::os::raw::c_char,
		activity: os_activity_t,
		flags: os_activity_flag_t,
	) -> os_activity_t;
}
extern "C" {
	pub fn _os_activity_label_useraction(
		dso: *mut ::std::os::raw::c_void,
		name: *const ::std::os::raw::c_char,
	);
}
extern "C" {
	pub fn _os_activity_initiate(
		dso: *mut ::std::os::raw::c_void,
		description: *const ::std::os::raw::c_char,
		flags: os_activity_flag_t,
		activity_block: os_block_t,
	);
}
extern "C" {
	pub fn _os_activity_initiate_f(
		dso: *mut ::std::os::raw::c_void,
		description: *const ::std::os::raw::c_char,
		flags: os_activity_flag_t,
		context: *mut ::std::os::raw::c_void,
		function: os_function_t,
	);
}
extern "C" {
	pub fn os_activity_apply(activity: os_activity_t, block: os_block_t);
}
extern "C" {
	pub fn os_activity_apply_f(
		activity: os_activity_t,
		context: *mut ::std::os::raw::c_void,
		function: os_function_t,
	);
}
extern "C" {
	pub fn os_activity_scope_enter(activity: os_activity_t, state: os_activity_scope_state_t);
}
extern "C" {
	pub fn os_activity_scope_leave(state: os_activity_scope_state_t);
}
extern "C" {
	pub fn os_activity_get_active(
		entries: *mut os_activity_id_t,
		count: *mut ::std::os::raw::c_uint,
	) -> ::std::os::raw::c_uint;
}
extern "C" {
	pub fn os_activity_get_identifier(
		activity: os_activity_t,
		parent_id: *mut os_activity_id_t,
	) -> os_activity_id_t;
}
extern "C" {
	pub fn _os_activity_start(
		dso: *mut ::std::os::raw::c_void,
		description: *const ::std::os::raw::c_char,
		flags: os_activity_flag_t,
	) -> os_activity_t;
}
extern "C" {
	pub fn os_activity_end(activity: os_activity_t);
}
extern "C" {
	pub fn _os_activity_set_breadcrumb(
		dso: *mut ::std::os::raw::c_void,
		name: *const ::std::os::raw::c_char,
	);
}
extern "C" {
	pub fn wrapped_get_default_log() -> os_log_t;
}
extern "C" {
	pub fn wrapped_os_log_with_type(
		log: os_log_t,
		type_: os_log_type_t,
		message: *const ::std::os::raw::c_char,
	);
}
extern "C" {
	pub fn wrapped_os_log_debug(log: os_log_t, message: *const ::std::os::raw::c_char);
}
extern "C" {
	pub fn wrapped_os_log_info(log: os_log_t, message: *const ::std::os::raw::c_char);
}
extern "C" {
	pub fn wrapped_os_log_default(log: os_log_t, message: *const ::std::os::raw::c_char);
}
extern "C" {
	pub fn wrapped_os_log_error(log: os_log_t, message: *const ::std::os::raw::c_char);
}
extern "C" {
	pub fn wrapped_os_log_fault(log: os_log_t, message: *const ::std::os::raw::c_char);
}
