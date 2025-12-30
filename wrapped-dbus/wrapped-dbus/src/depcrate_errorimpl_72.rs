// Generated macro for impl_72 (impl)
macro_rules! Depcrate_errorimpl_72 {
() => {
// Module: crate::error
// Provides: {"impl_72"}
// Dependencies: {}
impl From < MethodErr > for Error { fn from (t : MethodErr) -> Error { Error :: new_custom (t . errorname () , t . description ()) } }
};
}
