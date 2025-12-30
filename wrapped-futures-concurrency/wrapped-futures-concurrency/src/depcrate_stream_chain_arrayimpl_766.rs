// Generated macro for impl_766 (impl)
macro_rules! Depcrate_stream_chain_arrayimpl_766 {
() => {
// Module: crate::stream::chain::array
// Provides: {"impl_766"}
// Dependencies: {}
impl < S , const N : usize > fmt :: Debug for Chain < S , N > where S : Stream + fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . streams . iter ()) . finish () } }
};
}
