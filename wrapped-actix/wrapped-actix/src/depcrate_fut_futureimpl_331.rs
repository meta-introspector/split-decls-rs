// Generated macro for impl_331 (impl)
macro_rules! Depcrate_fut_futureimpl_331 {
() => {
// Module: crate::fut::future
// Provides: {"impl_331"}
// Dependencies: {}
impl < P , A > ActorFuture < A > for Pin < P > where P : Unpin + DerefMut , < P as Deref > :: Target : ActorFuture < A > , A : Actor , { type Output = < < P as Deref > :: Target as ActorFuture < A > > :: Output ; fn poll (self : Pin < & mut Self > , srv : & mut A , ctx : & mut A :: Context , task : & mut Context < '_ > ,) -> Poll < Self :: Output > { Pin :: get_mut (self) . as_mut () . poll (srv , ctx , task) } }
};
}
