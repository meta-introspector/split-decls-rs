// Generated macro for impl_18 (impl)
macro_rules! Depcrate_queueimpl_18 {
() => {
// Module: crate::queue
// Provides: {"impl_18"}
// Dependencies: {}
impl QueueState { fn run_all (& self) { let _was_scheduled = self . is_scheduled . replace (false) ; debug_assert ! (_was_scheduled) ; let mut task_count_left = self . tasks . borrow () . len () ; while task_count_left > 0 { task_count_left -= 1 ; let task = match self . tasks . borrow_mut () . pop_front () { Some (task) => task , None => break , } ; task . run () ; } } }
};
}
