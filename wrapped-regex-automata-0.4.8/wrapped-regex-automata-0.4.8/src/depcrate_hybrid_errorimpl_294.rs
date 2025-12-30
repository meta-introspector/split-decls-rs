// Generated macro for impl_294 (impl)
macro_rules! Depcrate_hybrid_errorimpl_294 {
() => {
// Module: crate::hybrid::error
// Provides: {"impl_294"}
// Dependencies: {}
# [cfg (feature = "std")] impl std :: error :: Error for BuildError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self . kind { BuildErrorKind :: NFA (ref err) => Some (err) , _ => None , } } }
};
}
