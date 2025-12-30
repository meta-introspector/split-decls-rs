// Generated macro for impl_71 (impl)
macro_rules! Depcrate_dfa_denseimpl_71 {
() => {
// Module: crate::dfa::dense
// Provides: {"impl_71"}
// Dependencies: {}
# [cfg (all (feature = "std" , feature = "dfa-build"))] impl std :: error :: Error for BuildError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self . kind () { BuildErrorKind :: NFA (ref err) => Some (err) , _ => None , } } }
};
}
