// Generated macro for impl_443 (impl)
macro_rules! Depcrate_future_try_joinimpl_443 {
() => {
// Module: crate::future::try_join
// Provides: {"impl_443"}
// Dependencies: {}
impl < Fut1 , Fut2 > Future for TryJoin < Fut1 , Fut2 > where Fut1 : TryFuture , Fut2 : TryFuture < Error = Fut1 :: Error > , { type Output = Result < (Fut1 :: Ok , Fut2 :: Ok) , Fut1 :: Error > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut all_done = true ; let mut futures = self . project () ; all_done &= futures . fut1 . as_mut () . poll (cx) ? . is_ready () ; all_done &= futures . fut2 . as_mut () . poll (cx) ? . is_ready () ; if all_done { Poll :: Ready (Ok ((futures . fut1 . take_output () . unwrap () , futures . fut2 . take_output () . unwrap () ,))) } else { Poll :: Pending } } }
};
}
