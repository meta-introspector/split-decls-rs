// Generated macro for impl_1216 (impl)
macro_rules! Depcrate_revision_spec_parse_delegateimpl_1216 {
() => {
// Module: crate::revision::spec::parse::delegate
// Provides: {"impl_1216"}
// Dependencies: {}
impl parse :: Delegate for Delegate < '_ > { fn done (& mut self) { self . follow_refs_to_objects_if_needed () ; self . disambiguate_objects_by_fallback_hint (self . kind_implies_committish () . then_some (ObjectKindHint :: Committish) . or (self . opts . object_kind_hint) ,) ; } }
};
}
