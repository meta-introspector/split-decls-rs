// Generated macro for remove_unreachable_tags (function)
macro_rules! Depcrate_provenance_gcremove_unreachable_tags {
() => {
// Module: crate::provenance_gc
// Provides: {"remove_unreachable_tags"}
// Dependencies: {}
fn remove_unreachable_tags < 'tcx > (ecx : & mut MiriInterpCx < 'tcx > , tags : FxHashSet < BorTag >) { if ecx . machine . borrow_tracker . is_some () { ecx . memory . alloc_map () . iter (| it | { for (_id , (_kind , alloc)) in it { alloc . extra . borrow_tracker . as_ref () . unwrap () . remove_unreachable_tags (& tags) ; } }) ; } }
};
}
