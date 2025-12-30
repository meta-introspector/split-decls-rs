// Generated macro for remove_unreachable_allocs (function)
macro_rules! Depcrate_provenance_gcremove_unreachable_allocs {
() => {
// Module: crate::provenance_gc
// Provides: {"remove_unreachable_allocs"}
// Dependencies: {}
fn remove_unreachable_allocs < 'tcx > (ecx : & mut MiriInterpCx < 'tcx > , allocs : FxHashSet < AllocId >) { let allocs = LiveAllocs { ecx , collected : allocs } ; ecx . machine . allocation_spans . borrow_mut () . retain (| id , _ | allocs . is_live (* id)) ; ecx . machine . symbolic_alignment . borrow_mut () . retain (| id , _ | allocs . is_live (* id)) ; ecx . machine . alloc_addresses . borrow_mut () . remove_unreachable_allocs (& allocs) ; if let Some (borrow_tracker) = & ecx . machine . borrow_tracker { borrow_tracker . borrow_mut () . remove_unreachable_allocs (& allocs) ; } ecx . remove_unreachable_allocs (& allocs . collected) ; }
};
}
