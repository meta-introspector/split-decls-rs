// Generated macro for impl_13 (impl)
macro_rules! Depcrate_errorimpl_13 {
() => {
// Module: crate::error
// Provides: {"impl_13"}
// Dependencies: {}
impl std :: error :: Error for FromEnvError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match & self . inner { FromEnvErrorInner :: CannotOpenPath (_ , err) => Some (err) , FromEnvErrorInner :: NotAPipe (_ , Some (err)) | FromEnvErrorInner :: CannotOpenFd (_ , err) => { Some (err) } _ => None , } } }
};
}
