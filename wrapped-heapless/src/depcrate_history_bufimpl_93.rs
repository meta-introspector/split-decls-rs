// Generated macro for impl_93 (impl)
macro_rules! Depcrate_history_bufimpl_93 {
() => {
// Module: crate::history_buf
// Provides: {"impl_93"}
// Dependencies: {}
impl < T , S : HistoryBufStorage < T > + ? Sized > fmt :: Debug for HistoryBufInner < T , S > where T : fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { < [T] as fmt :: Debug > :: fmt (self , f) } }
};
}
