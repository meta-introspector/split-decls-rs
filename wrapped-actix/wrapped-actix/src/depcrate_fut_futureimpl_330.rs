// Generated macro for impl_330 (impl)
macro_rules! Depcrate_fut_futureimpl_330 {
() => {
// Module: crate::fut::future
// Provides: {"impl_330"}
// Dependencies: {}
impl < F , A > ActorFuture < A > for Box < F > where F : ActorFuture < A > + Unpin + ? Sized , A : Actor , { type Output = F :: Output ; fn poll (mut self : Pin < & mut Self > , srv : & mut A , ctx : & mut A :: Context , task : & mut Context < '_ > ,) -> Poll < Self :: Output > { Pin :: new (& mut * * self . as_mut ()) . poll (srv , ctx , task) } }
};
}
