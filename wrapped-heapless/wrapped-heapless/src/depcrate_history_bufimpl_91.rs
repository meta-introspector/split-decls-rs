// Generated macro for impl_91 (impl)
macro_rules! Depcrate_history_bufimpl_91 {
() => {
// Module: crate::history_buf
// Provides: {"impl_91"}
// Dependencies: {}
impl < T , S : HistoryBufStorage < T > + ? Sized > Deref for HistoryBufInner < T , S > { type Target = [T] ; fn deref (& self) -> & [T] { self . as_slice () } }
};
}
