// Generated macro for impl_124 (impl)
macro_rules! Depcrate_borrow_tracker_stacked_borrows_diagnosticsimpl_124 {
() => {
// Module: crate::borrow_tracker::stacked_borrows::diagnostics
// Provides: {"impl_124"}
// Dependencies: {}
impl AllocHistory { pub fn new (id : AllocId , item : Item , machine : & MiriMachine < '_ >) -> Self { Self { id , root : (item , machine . current_user_relevant_span ()) , creations : SmallVec :: new () , invalidations : SmallVec :: new () , protectors : SmallVec :: new () , } } pub fn retain (& mut self , live_tags : & FxHashSet < BorTag >) { self . invalidations . retain (| event | live_tags . contains (& event . tag)) ; self . creations . retain (| event | live_tags . contains (& event . retag . new_tag)) ; self . protectors . retain (| event | live_tags . contains (& event . tag)) ; } }
};
}
