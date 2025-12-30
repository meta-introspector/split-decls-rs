// Generated macro for impl_1073 (impl)
macro_rules! Depcrate_stream_stream_flatten_unorderedimpl_1073 {
() => {
// Module: crate::stream::stream::flatten_unordered
// Provides: {"impl_1073"}
// Dependencies: {}
impl < F : FnOnce (& SharedPollState) -> u8 > Drop for PollStateBomb < '_ , F > { fn drop (& mut self) { if let Some (drop) = self . drop . take () { (drop) (self . state) ; } } }
};
}
