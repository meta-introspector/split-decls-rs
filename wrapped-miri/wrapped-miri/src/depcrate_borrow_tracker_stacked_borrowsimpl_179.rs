// Generated macro for impl_179 (impl)
macro_rules! Depcrate_borrow_tracker_stacked_borrowsimpl_179 {
() => {
// Module: crate::borrow_tracker::stacked_borrows
// Provides: {"impl_179"}
// Dependencies: {}
# [doc = " Integration with the BorTag garbage collector"] impl Stacks { pub fn remove_unreachable_tags (& mut self , live_tags : & FxHashSet < BorTag >) { for (_stack_range , stack) in self . stacks . iter_mut_all () { stack . retain (live_tags) ; } self . history . retain (live_tags) ; } }
};
}
