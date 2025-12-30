// Generated macro for impl_110 (impl)
macro_rules! Depcrate_dfa_onepassimpl_110 {
() => {
// Module: crate::dfa::onepass
// Provides: {"impl_110"}
// Dependencies: {}
# [cfg (feature = "std")] impl std :: error :: Error for BuildError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { use self :: BuildErrorKind :: * ; match self . kind { NFA (ref err) => Some (err) , Word (ref err) => Some (err) , _ => None , } } }
};
}
