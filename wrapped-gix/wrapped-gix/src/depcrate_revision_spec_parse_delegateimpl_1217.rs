// Generated macro for impl_1217 (impl)
macro_rules! Depcrate_revision_spec_parse_delegateimpl_1217 {
() => {
// Module: crate::revision::spec::parse::delegate
// Provides: {"impl_1217"}
// Dependencies: {}
impl delegate :: Kind for Delegate < '_ > { fn kind (& mut self , kind : gix_revision :: spec :: Kind) -> Option < () > { use gix_revision :: spec :: Kind :: * ; self . kind = Some (kind) ; if self . kind_implies_committish () { self . disambiguate_objects_by_fallback_hint (ObjectKindHint :: Committish . into ()) ; } if matches ! (kind , RangeBetween | ReachableToMergeBase) { self . idx += 1 ; } Some (()) } }
};
}
