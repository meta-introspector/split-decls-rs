// Generated macro for arg_append_str (function)
macro_rules! Depcrate_arg_basic_implarg_append_str {
() => {
// Module: crate::arg::basic_impl
// Provides: {"arg_append_str"}
// Dependencies: {}
fn arg_append_str (i : * mut ffi :: DBusMessageIter , arg_type : ArgType , v : & CStr) { let p = v . as_ptr () ; let q = & p as * const _ as * const c_void ; unsafe { check ("dbus_message_iter_append_basic" , ffi :: dbus_message_iter_append_basic (i , arg_type as c_int , q)) ; } ; }
};
}
