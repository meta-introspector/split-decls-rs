// Generated macro for impl_449 (impl)
macro_rules! Depcrate_fut_try_future_map_okimpl_449 {
() => {
// Module: crate::fut::try_future::map_ok
// Provides: {"impl_449"}
// Dependencies: {}
impl < U , Fut , A , F > ActorFuture < A > for MapOk < Fut , F > where Fut : ActorTryFuture < A > , A : Actor , F : FnOnce (Fut :: Ok , & mut A , & mut A :: Context) -> U , { type Output = Result < U , Fut :: Error > ; fn poll (mut self : Pin < & mut Self > , act : & mut A , ctx : & mut A :: Context , task : & mut Context < '_ > ,) -> Poll < Self :: Output > { match self . as_mut () . project () { MapProj :: Incomplete { future , .. } => { let output = ready ! (future . try_poll (act , ctx , task)) ; match self . project_replace (MapOk :: Complete) { MapProjReplace :: Incomplete { f , .. } => { Poll :: Ready (output . map (| ok | f (ok , act , ctx))) } MapProjReplace :: Complete => unreachable ! () , } } MapProj :: Complete => { panic ! ("MapOk must not be polled after it returned `Poll::Ready`") } } } }
};
}
