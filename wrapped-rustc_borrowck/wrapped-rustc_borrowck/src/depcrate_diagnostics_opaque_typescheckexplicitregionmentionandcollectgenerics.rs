// Generated macro for CheckExplicitRegionMentionAndCollectGenerics (struct)
macro_rules! Depcrate_diagnostics_opaque_typesCheckExplicitRegionMentionAndCollectGenerics {
() => {
// Module: crate::diagnostics::opaque_types
// Provides: {"CheckExplicitRegionMentionAndCollectGenerics"}
// Dependencies: {}
struct CheckExplicitRegionMentionAndCollectGenerics < 'tcx > { tcx : TyCtxt < 'tcx > , generics : & 'tcx ty :: Generics , offending_region_idx : usize , seen_opaques : FxIndexSet < DefId > , seen_lifetimes : FxIndexSet < DefId > , }
};
}
