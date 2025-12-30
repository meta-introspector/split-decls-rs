// Generated macro for impl_66 (impl)
macro_rules! Depcrate_errorimpl_66 {
() => {
// Module: crate::error
// Provides: {"impl_66"}
// Dependencies: {}
impl Error { # [doc = " Create a new custom D-Bus Error."] pub fn new_custom < 'a , N : Into < ErrorName < 'a > > > (name : N , message : & str) -> Error { let n = to_c_str (& name . into ()) ; let m = to_c_str (& message . replace ("%" , "%%")) ; let mut e = Error :: empty () ; unsafe { ffi :: dbus_set_error (e . get_mut () , n . as_ptr () , m . as_ptr ()) } ; e } # [doc = " Create a new generic D-Bus Error with \"org.freedesktop.DBus.Error.Failed\" as the Error name."] pub fn new_failed (message : & str) -> Error { Error :: new_custom ("org.freedesktop.DBus.Error.Failed" , message) } pub (crate) fn empty () -> Error { init_dbus () ; let mut e = ffi :: DBusError { name : ptr :: null () , message : ptr :: null () , dummy : 0 , padding1 : ptr :: null () } ; unsafe { ffi :: dbus_error_init (& mut e) ; } Error { e : e } } # [doc = " Error name/type, e g 'org.freedesktop.DBus.Error.Failed'"] pub fn name (& self) -> Option < & str > { c_str_to_slice (& self . e . name) } # [doc = " Custom message, e g 'Could not find a matching object path'"] pub fn message (& self) -> Option < & str > { c_str_to_slice (& self . e . message) } pub (crate) fn get_mut (& mut self) -> & mut ffi :: DBusError { & mut self . e } }
};
}
