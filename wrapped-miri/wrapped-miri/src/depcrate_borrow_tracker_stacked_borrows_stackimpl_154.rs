// Generated macro for impl_154 (impl)
macro_rules! Depcrate_borrow_tracker_stacked_borrows_stackimpl_154 {
() => {
// Module: crate::borrow_tracker::stacked_borrows::stack
// Provides: {"impl_154"}
// Dependencies: {}
impl PartialEq for Stack { fn eq (& self , other : & Self) -> bool { let Stack { borrows , unknown_bottom , # [cfg (feature = "stack-cache")] cache : _ , # [cfg (feature = "stack-cache")] unique_range : _ , } = self ; * borrows == other . borrows && * unknown_bottom == other . unknown_bottom } }
};
}
