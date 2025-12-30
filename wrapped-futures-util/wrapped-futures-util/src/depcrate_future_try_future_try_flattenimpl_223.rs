// Generated macro for impl_223 (impl)
macro_rules! Depcrate_future_try_future_try_flattenimpl_223 {
() => {
// Module: crate::future::try_future::try_flatten
// Provides: {"impl_223"}
// Dependencies: {}
impl < Fut > Future for TryFlatten < Fut , Fut :: Ok > where Fut : TryFuture , Fut :: Ok : TryFuture < Error = Fut :: Error > , { type Output = Result < < Fut :: Ok as TryFuture > :: Ok , Fut :: Error > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { Poll :: Ready (loop { match self . as_mut () . project () { TryFlattenProj :: First { f } => match ready ! (f . try_poll (cx)) { Ok (f) => self . set (Self :: Second { f }) , Err (e) => { self . set (Self :: Empty) ; break Err (e) ; } } , TryFlattenProj :: Second { f } => { let output = ready ! (f . try_poll (cx)) ; self . set (Self :: Empty) ; break output ; } TryFlattenProj :: Empty => panic ! ("TryFlatten polled after completion") , } }) } }
};
}
