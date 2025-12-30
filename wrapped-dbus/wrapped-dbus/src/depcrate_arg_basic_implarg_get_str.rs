// Generated macro for arg_get_str (function)
macro_rules! Depcrate_arg_basic_implarg_get_str {
() => {
// Module: crate::arg::basic_impl
// Provides: {"arg_get_str"}
// Dependencies: {}
unsafe fn arg_get_str < 'a > (i : * mut ffi :: DBusMessageIter , arg_type : ArgType) -> Option < & 'a CStr > { if ffi :: dbus_message_iter_get_arg_type (i) != arg_type as c_int { return None } ; let mut p = ptr :: null_mut () ; ffi :: dbus_message_iter_get_basic (i , & mut p as * mut _ as * mut c_void) ; Some (CStr :: from_ptr (p as * const c_char)) }
};
}
