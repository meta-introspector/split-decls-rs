// Generated macro for impl_88 (impl)
macro_rules! Depcrate_history_bufimpl_88 {
() => {
// Module: crate::history_buf
// Provides: {"impl_88"}
// Dependencies: {}
impl < 'a , T , S : HistoryBufStorage < T > + ? Sized > Extend < & 'a T > for HistoryBufInner < T , S > where T : 'a + Clone , { fn extend < I > (& mut self , iter : I) where I : IntoIterator < Item = & 'a T > , { self . extend (iter . into_iter () . cloned ()) ; } }
};
}
