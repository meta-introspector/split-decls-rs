// Generated macro for impl_378 (impl)
macro_rules! Depcrate_borrow_trackerimpl_378 {
() => {
// Module: crate::borrow_tracker
// Provides: {"impl_378"}
// Dependencies: {}
impl BorrowTrackerMethod { pub fn instantiate_global_state (self , config : & MiriConfig) -> GlobalState { RefCell :: new (GlobalStateInner :: new (self , config . tracked_pointer_tags . clone ())) } pub fn get_tree_borrows_params (self) -> TreeBorrowsParams { match self { BorrowTrackerMethod :: TreeBorrows (params) => params , _ => panic ! ("can only be called when `BorrowTrackerMethod` is `TreeBorrows`") , } } }
};
}
