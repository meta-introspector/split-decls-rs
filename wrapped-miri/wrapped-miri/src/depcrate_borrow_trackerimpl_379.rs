// Generated macro for impl_379 (impl)
macro_rules! Depcrate_borrow_trackerimpl_379 {
() => {
// Module: crate::borrow_tracker
// Provides: {"impl_379"}
// Dependencies: {}
impl GlobalStateInner { pub fn new_allocation (& mut self , id : AllocId , alloc_size : Size , kind : MemoryKind , machine : & MiriMachine < '_ > ,) -> AllocState { let _trace = enter_trace_span ! (borrow_tracker :: new_allocation , ? id , ? alloc_size , ? kind) ; match self . borrow_tracker_method { BorrowTrackerMethod :: StackedBorrows => AllocState :: StackedBorrows (Box :: new (RefCell :: new (Stacks :: new_allocation (id , alloc_size , self , kind , machine ,)))) , BorrowTrackerMethod :: TreeBorrows { .. } => AllocState :: TreeBorrows (Box :: new (RefCell :: new (Tree :: new_allocation (id , alloc_size , self , kind , machine ,)))) , } } }
};
}
