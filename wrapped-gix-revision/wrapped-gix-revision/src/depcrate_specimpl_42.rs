// Generated macro for impl_42 (impl)
macro_rules! Depcrate_specimpl_42 {
() => {
// Module: crate::spec
// Provides: {"impl_42"}
// Dependencies: {}
impl Spec { # [doc = " Return the kind of this specification."] pub fn kind (& self) -> Kind { match self { Spec :: Include (_) => Kind :: IncludeReachable , Spec :: Exclude (_) => Kind :: ExcludeReachable , Spec :: Range { .. } => Kind :: RangeBetween , Spec :: Merge { .. } => Kind :: ReachableToMergeBase , Spec :: IncludeOnlyParents { .. } => Kind :: IncludeReachableFromParents , Spec :: ExcludeParents { .. } => Kind :: ExcludeReachableFromParents , } } }
};
}
