// Generated macro for impl_79 (impl)
macro_rules! Depcrate_errorimpl_79 {
() => {
// Module: crate::error
// Provides: {"impl_79"}
// Dependencies: {}
impl From < Error > for MethodErr { fn from (t : Error) -> MethodErr { let n = t . name () . unwrap_or ("org.freedesktop.DBus.Error.Failed") ; let m = t . message () . unwrap_or ("Unknown error") ; MethodErr (String :: from (n) . into () , m . into ()) } }
};
}
