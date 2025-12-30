// Generated macro for impl_83 (impl)
macro_rules! Depcrate_parallel_reduceimpl_83 {
() => {
// Module: crate::parallel::reduce
// Provides: {"impl_83"}
// Dependencies: {}
impl < Input , Error > Reduce for IdentityWithResult < Input , Error > { type Input = Result < Input , Self :: Error > ; type FeedProduce = Input ; type Output = () ; type Error = Error ; fn feed (& mut self , item : Self :: Input) -> Result < Self :: FeedProduce , Self :: Error > { item } fn finalize (self) -> Result < Self :: Output , Self :: Error > { Ok (()) } }
};
}
