// Generated macro for impl_87 (impl)
macro_rules! Depcrate_history_bufimpl_87 {
() => {
// Module: crate::history_buf
// Provides: {"impl_87"}
// Dependencies: {}
impl < T , S : HistoryBufStorage < T > + ? Sized > Extend < T > for HistoryBufInner < T , S > { fn extend < I > (& mut self , iter : I) where I : IntoIterator < Item = T > , { for item in iter . into_iter () { self . write (item) ; } } }
};
}
