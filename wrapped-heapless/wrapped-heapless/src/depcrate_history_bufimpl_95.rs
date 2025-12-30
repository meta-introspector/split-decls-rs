// Generated macro for impl_95 (impl)
macro_rules! Depcrate_history_bufimpl_95 {
() => {
// Module: crate::history_buf
// Provides: {"impl_95"}
// Dependencies: {}
impl < T , S : HistoryBufStorage < T > + ? Sized > PartialEq for HistoryBufInner < T , S > where T : PartialEq , { fn eq (& self , other : & Self) -> bool { self . oldest_ordered () . eq (other . oldest_ordered ()) } }
};
}
