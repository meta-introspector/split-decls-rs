// Generated macro for impl_67 (impl)
macro_rules! Depcrate_errorimpl_67 {
() => {
// Module: crate::error
// Provides: {"impl_67"}
// Dependencies: {}
impl Drop for Error { fn drop (& mut self) { unsafe { ffi :: dbus_error_free (& mut self . e) ; } } }
};
}
