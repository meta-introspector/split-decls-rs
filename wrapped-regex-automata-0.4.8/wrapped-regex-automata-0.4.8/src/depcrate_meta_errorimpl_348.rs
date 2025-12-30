// Generated macro for impl_348 (impl)
macro_rules! Depcrate_meta_errorimpl_348 {
() => {
// Module: crate::meta::error
// Provides: {"impl_348"}
// Dependencies: {}
# [cfg (feature = "std")] impl std :: error :: Error for BuildError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self . kind { BuildErrorKind :: Syntax { ref err , .. } => Some (err) , BuildErrorKind :: NFA (ref err) => Some (err) , } } }
};
}
