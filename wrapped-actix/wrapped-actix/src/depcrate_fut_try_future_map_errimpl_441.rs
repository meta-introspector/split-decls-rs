// Generated macro for impl_441 (impl)
macro_rules! Depcrate_fut_try_future_map_errimpl_441 {
() => {
// Module: crate::fut::try_future::map_err
// Provides: {"impl_441"}
// Dependencies: {}
impl < U , Fut , A , F > ActorFuture < A > for MapErr < Fut , F > where Fut : ActorTryFuture < A > , A : Actor , F : FnOnce (Fut :: Error , & mut A , & mut A :: Context) -> U , { type Output = Result < Fut :: Ok , U > ; fn poll (mut self : Pin < & mut Self > , act : & mut A , ctx : & mut A :: Context , task : & mut Context < '_ > ,) -> Poll < Self :: Output > { match self . as_mut () . project () { MapProj :: Incomplete { future , .. } => { let output = ready ! (future . try_poll (act , ctx , task)) ; match self . project_replace (MapErr :: Complete) { MapProjReplace :: Incomplete { f , .. } => { Poll :: Ready (output . map_err (| err | f (err , act , ctx))) } MapProjReplace :: Complete => unreachable ! () , } } MapProj :: Complete => { panic ! ("MapErr must not be polled after it returned `Poll::Ready`") } } } }
};
}
