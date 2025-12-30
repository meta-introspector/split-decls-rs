// Generated macro for impl_222 (impl)
macro_rules! Depcrate_handlerimpl_222 {
() => {
// Module: crate::handler
// Provides: {"impl_222"}
// Dependencies: {}
impl < A : Actor , I > From < Pin < Box < dyn ActorFuture < A , Output = I > > > > for ActorResponse < A , I > { fn from (fut : Pin < Box < dyn ActorFuture < A , Output = I > > >) -> Self { Self { item : ActorResponseTypeItem :: Fut (fut) , } } }
};
}
