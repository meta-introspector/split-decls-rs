// Generated macro for impl_343 (impl)
macro_rules! Depcrate_borrow_trackerimpl_343 {
() => {
// Module: crate::borrow_tracker
// Provides: {"impl_343"}
// Dependencies: {}
impl BorrowTrackerMethod { pub fn instantiate_global_state (self , config : & MiriConfig) -> GlobalState { RefCell :: new (GlobalStateInner :: new (self , config . tracked_pointer_tags . clone () , config . retag_fields ,)) } pub fn get_tree_borrows_params (self) -> TreeBorrowsParams { match self { BorrowTrackerMethod :: TreeBorrows (params) => params , _ => panic ! ("can only be called when `BorrowTrackerMethod` is `TreeBorrows`") , } } }
};
}
