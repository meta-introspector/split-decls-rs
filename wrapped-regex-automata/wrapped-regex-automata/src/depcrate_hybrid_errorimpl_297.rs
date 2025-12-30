// Generated macro for impl_297 (impl)
macro_rules! Depcrate_hybrid_errorimpl_297 {
() => {
// Module: crate::hybrid::error
// Provides: {"impl_297"}
// Dependencies: {}
# [cfg (feature = "std")] impl std :: error :: Error for BuildError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self . kind { BuildErrorKind :: NFA (ref err) => Some (err) , _ => None , } } }
};
}
