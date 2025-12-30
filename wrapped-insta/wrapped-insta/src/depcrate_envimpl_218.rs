// Generated macro for impl_218 (impl)
macro_rules! Depcrate_envimpl_218 {
() => {
// Module: crate::env
// Provides: {"impl_218"}
// Dependencies: {}
impl std :: error :: Error for Error { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self { Error :: Deserialize (ref err) => Some (err) , _ => None , } } }
};
}
