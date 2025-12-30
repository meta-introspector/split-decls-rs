// Generated macro for impl_298 (impl)
macro_rules! Depcrate_hybrid_errorimpl_298 {
() => {
// Module: crate::hybrid::error
// Provides: {"impl_298"}
// Dependencies: {}
# [cfg (feature = "std")] impl std :: error :: Error for StartError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match * self { StartError :: Cache { ref err } => Some (err) , _ => None , } } }
};
}
