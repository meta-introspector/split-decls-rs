// Generated macro for impl_302 (impl)
macro_rules! Depcrate_fut_future_mapimpl_302 {
() => {
// Module: crate::fut::future::map
// Provides: {"impl_302"}
// Dependencies: {}
impl < U , Fut , A , F > ActorFuture < A > for Map < Fut , F > where Fut : ActorFuture < A > , A : Actor , F : FnOnce (Fut :: Output , & mut A , & mut A :: Context) -> U , { type Output = U ; fn poll (mut self : Pin < & mut Self > , act : & mut A , ctx : & mut A :: Context , task : & mut Context < '_ > ,) -> Poll < Self :: Output > { match self . as_mut () . project () { MapProj :: Incomplete { future , .. } => { let output = ready ! (future . poll (act , ctx , task)) ; match self . project_replace (Map :: Complete) { MapProjReplace :: Incomplete { f , .. } => Poll :: Ready (f (output , act , ctx)) , MapProjReplace :: Complete => unreachable ! () , } } MapProj :: Complete => { panic ! ("Map must not be polled after it returned `Poll::Ready`") } } } }
};
}
