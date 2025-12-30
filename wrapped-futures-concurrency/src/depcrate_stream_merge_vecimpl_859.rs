// Generated macro for impl_859 (impl)
macro_rules! Depcrate_stream_merge_vecimpl_859 {
() => {
// Module: crate::stream::merge::vec
// Provides: {"impl_859"}
// Dependencies: {}
impl < S > fmt :: Debug for Merge < S > where S : Stream + fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . streams . iter ()) . finish () } }
};
}
