// Generated macro for impl_1191 (impl)
macro_rules! Depcrate_test_runner_replayimpl_1191 {
() => {
// Module: crate::test_runner::replay
// Provides: {"impl_1191"}
// Dependencies: {}
impl Replay { # [doc = " If `other` is longer than `self`, add the extra elements to `self`."] pub fn merge (& mut self , other : & Replay) { if other . steps . len () > self . steps . len () { let sl = self . steps . len () ; self . steps . extend_from_slice (& other . steps [sl ..]) ; } } }
};
}
