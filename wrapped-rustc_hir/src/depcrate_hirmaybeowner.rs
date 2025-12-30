// Generated macro for MaybeOwner (enum)
macro_rules! Depcrate_hirMaybeOwner {
() => {
// Module: crate::hir
// Provides: {"MaybeOwner"}
// Dependencies: {}
# [derive (Copy , Clone , Debug , HashStable_Generic)] pub enum MaybeOwner < 'tcx > { Owner (& 'tcx OwnerInfo < 'tcx >) , NonOwner (HirId) , # [doc = " Used as a placeholder for unused LocalDefId."] Phantom , }
};
}
