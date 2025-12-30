// Generated macro for impl_194 (impl)
macro_rules! Depcrate_testimpl_194 {
() => {
// Module: crate::test
// Provides: {"impl_194"}
// Dependencies: {}
impl WaitState { # [doc = " Ensure the received trackers count is equal to the provided one"] pub fn ensure_trackers_len (self , len : usize) -> Self { assert_eq ! (self . trackers . len () , len , "Unexpected cookies len. events: {:#?}" , self . received) ; self } # [doc = " Ensure there is no one event after expected"] pub fn ensure_no_tail (self) -> Self { assert ! (self . remain . is_empty () , "Unexpected events from the watcher: unexpected: {:#?}" , self . remain) ; self } }
};
}
