// Generated macro for impl_85 (impl)
macro_rules! Depcrate_history_bufimpl_85 {
() => {
// Module: crate::history_buf
// Provides: {"impl_85"}
// Dependencies: {}
impl < T , S : HistoryBufStorage < T > + ? Sized > HistoryBufInner < T , S > { # [doc = " Clears the buffer"] pub fn clear (& mut self) { unsafe { self . drop_contents () } ; self . write_at = 0 ; self . filled = false ; } }
};
}
