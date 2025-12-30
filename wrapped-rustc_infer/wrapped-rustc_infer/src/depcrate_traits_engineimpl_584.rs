// Generated macro for impl_584 (impl)
macro_rules! Depcrate_traits_engineimpl_584 {
() => {
// Module: crate::traits::engine
// Provides: {"impl_584"}
// Dependencies: {}
impl < 'tcx > ScrubbedTraitError < 'tcx > { pub fn is_true_error (& self) -> bool { match self { ScrubbedTraitError :: TrueError => true , ScrubbedTraitError :: Ambiguity | ScrubbedTraitError :: Cycle (_) => false , } } }
};
}
