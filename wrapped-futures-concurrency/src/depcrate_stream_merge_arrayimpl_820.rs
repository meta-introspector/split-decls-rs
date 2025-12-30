// Generated macro for impl_820 (impl)
macro_rules! Depcrate_stream_merge_arrayimpl_820 {
() => {
// Module: crate::stream::merge::array
// Provides: {"impl_820"}
// Dependencies: {}
impl < S , const N : usize > fmt :: Debug for Merge < S , N > where S : Stream + fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . streams . iter ()) . finish () } }
};
}
