// Generated macro for impl_801 (impl)
macro_rules! Depcrate_stream_chain_vecimpl_801 {
() => {
// Module: crate::stream::chain::vec
// Provides: {"impl_801"}
// Dependencies: {}
impl < S : Stream > ChainTrait for Vec < S > { type Item = S :: Item ; type Stream = Chain < S > ; fn chain (self) -> Self :: Stream { Chain { len : self . len () , streams : self , index : 0 , done : false , } } }
};
}
