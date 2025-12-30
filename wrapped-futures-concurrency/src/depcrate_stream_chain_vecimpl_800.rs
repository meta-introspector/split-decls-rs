// Generated macro for impl_800 (impl)
macro_rules! Depcrate_stream_chain_vecimpl_800 {
() => {
// Module: crate::stream::chain::vec
// Provides: {"impl_800"}
// Dependencies: {}
impl < S > fmt :: Debug for Chain < S > where S : Stream + fmt :: Debug , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_list () . entries (self . streams . iter ()) . finish () } }
};
}
