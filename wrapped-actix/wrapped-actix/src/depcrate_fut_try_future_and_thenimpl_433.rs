// Generated macro for impl_433 (impl)
macro_rules! Depcrate_fut_try_future_and_thenimpl_433 {
() => {
// Module: crate::fut::try_future::and_then
// Provides: {"impl_433"}
// Dependencies: {}
impl < A , B , F , Act > ActorFuture < Act > for AndThen < A , B , F > where A : ActorTryFuture < Act > , B : ActorTryFuture < Act , Error = A :: Error > , F : FnOnce (A :: Ok , & mut Act , & mut Act :: Context) -> B , Act : Actor , { type Output = Result < B :: Ok , A :: Error > ; fn poll (mut self : Pin < & mut Self > , act : & mut Act , ctx : & mut Act :: Context , task : & mut Context < '_ > ,) -> Poll < Self :: Output > { match self . as_mut () . project () { AndThenProj :: First { fut1 , data } => { let ok = ready ! (fut1 . try_poll (act , ctx , task)) ? ; let data = data . take () . unwrap () ; let fut2 = data (ok , act , ctx) ; self . set (AndThen :: Second { fut2 }) ; self . poll (act , ctx , task) } AndThenProj :: Second { fut2 } => { let res = ready ! (fut2 . try_poll (act , ctx , task)) ; self . set (AndThen :: Empty) ; Poll :: Ready (res) } AndThenProj :: Empty => panic ! ("ActorFuture polled after finish") , } } }
};
}
