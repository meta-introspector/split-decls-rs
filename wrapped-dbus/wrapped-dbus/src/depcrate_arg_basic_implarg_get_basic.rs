// Generated macro for arg_get_basic (function)
macro_rules! Depcrate_arg_basic_implarg_get_basic {
() => {
// Module: crate::arg::basic_impl
// Provides: {"arg_get_basic"}
// Dependencies: {}
fn arg_get_basic < T > (i : * mut ffi :: DBusMessageIter , arg_type : ArgType) -> Option < T > { unsafe { if ffi :: dbus_message_iter_get_arg_type (i) != arg_type as c_int { return None } ; let mut c = mem :: MaybeUninit :: uninit () ; ffi :: dbus_message_iter_get_basic (i , & mut c as * mut _ as * mut c_void) ; Some (c . assume_init ()) } }
};
}
