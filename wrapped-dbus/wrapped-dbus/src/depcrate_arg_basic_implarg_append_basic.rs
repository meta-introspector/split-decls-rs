// Generated macro for arg_append_basic (function)
macro_rules! Depcrate_arg_basic_implarg_append_basic {
() => {
// Module: crate::arg::basic_impl
// Provides: {"arg_append_basic"}
// Dependencies: {}
fn arg_append_basic < T > (i : * mut ffi :: DBusMessageIter , arg_type : ArgType , v : T) { let p = & v as * const _ as * const c_void ; unsafe { check ("dbus_message_iter_append_basic" , ffi :: dbus_message_iter_append_basic (i , arg_type as c_int , p)) ; } ; }
};
}
