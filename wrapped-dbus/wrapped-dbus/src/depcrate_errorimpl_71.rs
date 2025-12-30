// Generated macro for impl_71 (impl)
macro_rules! Depcrate_errorimpl_71 {
() => {
// Module: crate::error
// Provides: {"impl_71"}
// Dependencies: {}
impl From < arg :: TypeMismatchError > for Error { fn from (t : arg :: TypeMismatchError) -> Error { Error :: new_custom ("org.freedesktop.DBus.Error.Failed" , & format ! ("{}" , t)) } }
};
}
