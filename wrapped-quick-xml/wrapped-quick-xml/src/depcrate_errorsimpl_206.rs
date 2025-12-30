// Generated macro for impl_206 (impl)
macro_rules! Depcrate_errorsimpl_206 {
() => {
// Module: crate::errors
// Provides: {"impl_206"}
// Dependencies: {}
impl std :: error :: Error for Error { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self { Self :: Io (e) => Some (e) , Self :: Syntax (e) => Some (e) , Self :: IllFormed (e) => Some (e) , Self :: InvalidAttr (e) => Some (e) , Self :: Encoding (e) => Some (e) , Self :: Escape (e) => Some (e) , Self :: Namespace (e) => Some (e) , } } }
};
}
