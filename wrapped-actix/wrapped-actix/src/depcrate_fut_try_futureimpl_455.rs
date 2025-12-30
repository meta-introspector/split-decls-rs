// Generated macro for impl_455 (impl)
macro_rules! Depcrate_fut_try_futureimpl_455 {
() => {
// Module: crate::fut::try_future
// Provides: {"impl_455"}
// Dependencies: {}
impl < A , F , T , E > ActorTryFuture < A > for F where A : Actor , F : ActorFuture < A , Output = Result < T , E > > + ? Sized , { type Ok = T ; type Error = E ; fn try_poll (self : Pin < & mut Self > , srv : & mut A , ctx : & mut A :: Context , task : & mut Context < '_ > ,) -> Poll < F :: Output > { self . poll (srv , ctx , task) } }
};
}
