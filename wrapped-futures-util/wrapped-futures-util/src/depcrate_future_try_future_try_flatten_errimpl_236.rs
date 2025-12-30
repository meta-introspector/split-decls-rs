// Generated macro for impl_236 (impl)
macro_rules! Depcrate_future_try_future_try_flatten_errimpl_236 {
() => {
// Module: crate::future::try_future::try_flatten_err
// Provides: {"impl_236"}
// Dependencies: {}
impl < Fut > Future for TryFlattenErr < Fut , Fut :: Error > where Fut : TryFuture , Fut :: Error : TryFuture < Ok = Fut :: Ok > , { type Output = Result < Fut :: Ok , < Fut :: Error as TryFuture > :: Error > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { Poll :: Ready (loop { match self . as_mut () . project () { TryFlattenErrProj :: First { f } => match ready ! (f . try_poll (cx)) { Err (f) => self . set (Self :: Second { f }) , Ok (e) => { self . set (Self :: Empty) ; break Ok (e) ; } } , TryFlattenErrProj :: Second { f } => { let output = ready ! (f . try_poll (cx)) ; self . set (Self :: Empty) ; break output ; } TryFlattenErrProj :: Empty => panic ! ("TryFlattenErr polled after completion") , } }) } }
};
}
