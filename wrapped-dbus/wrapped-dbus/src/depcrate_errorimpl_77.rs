// Generated macro for impl_77 (impl)
macro_rules! Depcrate_errorimpl_77 {
() => {
// Module: crate::error
// Provides: {"impl_77"}
// Dependencies: {}
impl From < TypeMismatchError > for MethodErr { fn from (t : TypeMismatchError) -> MethodErr { ("org.freedesktop.DBus.Error.Failed" , format ! ("{}" , t)) . into () } }
};
}
