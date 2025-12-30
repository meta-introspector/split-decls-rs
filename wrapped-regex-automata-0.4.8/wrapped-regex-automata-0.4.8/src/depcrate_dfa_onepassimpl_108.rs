// Generated macro for impl_108 (impl)
macro_rules! Depcrate_dfa_onepassimpl_108 {
() => {
// Module: crate::dfa::onepass
// Provides: {"impl_108"}
// Dependencies: {}
# [cfg (feature = "std")] impl std :: error :: Error for BuildError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { use self :: BuildErrorKind :: * ; match self . kind { NFA (ref err) => Some (err) , Word (ref err) => Some (err) , _ => None , } } }
};
}
