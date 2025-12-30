// Generated macro for AllocHistory (struct)
macro_rules! Depcrate_borrow_tracker_stacked_borrows_diagnosticsAllocHistory {
() => {
// Module: crate::borrow_tracker::stacked_borrows::diagnostics
// Provides: {"AllocHistory"}
// Dependencies: {}
# [derive (Clone , Debug)] pub struct AllocHistory { id : AllocId , root : (Item , Span) , creations : smallvec :: SmallVec < [Creation ; 1] > , invalidations : smallvec :: SmallVec < [Invalidation ; 1] > , protectors : smallvec :: SmallVec < [Protection ; 1] > , }
};
}
