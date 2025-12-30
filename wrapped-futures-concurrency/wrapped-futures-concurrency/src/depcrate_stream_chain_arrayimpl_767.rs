// Generated macro for impl_767 (impl)
macro_rules! Depcrate_stream_chain_arrayimpl_767 {
() => {
// Module: crate::stream::chain::array
// Provides: {"impl_767"}
// Dependencies: {}
impl < S : Stream , const N : usize > ChainTrait for [S ; N] { type Item = S :: Item ; type Stream = Chain < S , N > ; fn chain (self) -> Self :: Stream { Chain { len : self . len () , streams : self , index : 0 , done : false , } } }
};
}
