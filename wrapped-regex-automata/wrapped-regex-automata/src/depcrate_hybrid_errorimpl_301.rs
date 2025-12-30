// Generated macro for impl_301 (impl)
macro_rules! Depcrate_hybrid_errorimpl_301 {
() => {
// Module: crate::hybrid::error
// Provides: {"impl_301"}
// Dependencies: {}
# [cfg (feature = "std")] impl std :: error :: Error for StartError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match * self { StartError :: Cache { ref err } => Some (err) , _ => None , } } }
};
}
