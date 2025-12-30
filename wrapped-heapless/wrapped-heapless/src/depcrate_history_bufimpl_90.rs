// Generated macro for impl_90 (impl)
macro_rules! Depcrate_history_bufimpl_90 {
() => {
// Module: crate::history_buf
// Provides: {"impl_90"}
// Dependencies: {}
impl < T , S : HistoryBufStorage < T > + ? Sized > Drop for HistoryBufInner < T , S > { fn drop (& mut self) { unsafe { self . drop_contents () } } }
};
}
