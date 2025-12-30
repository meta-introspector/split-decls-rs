// Generated macro for impl_318 (impl)
macro_rules! Depcrate_fut_future_thenimpl_318 {
() => {
// Module: crate::fut::future::then
// Provides: {"impl_318"}
// Dependencies: {}
impl < A , B , F , Act > ActorFuture < Act > for Then < A , B , F > where A : ActorFuture < Act > , B : ActorFuture < Act > , F : FnOnce (A :: Output , & mut Act , & mut Act :: Context) -> B , Act : Actor , { type Output = B :: Output ; fn poll (mut self : Pin < & mut Self > , act : & mut Act , ctx : & mut Act :: Context , task : & mut task :: Context < '_ > ,) -> Poll < B :: Output > { match self . as_mut () . project () { ThenProj :: First { fut1 , data } => { let output = ready ! (fut1 . poll (act , ctx , task)) ; let data = data . take () . unwrap () ; let fut2 = data (output , act , ctx) ; self . set (Then :: Second { fut2 }) ; self . poll (act , ctx , task) } ThenProj :: Second { fut2 } => { let res = ready ! (fut2 . poll (act , ctx , task)) ; self . set (Then :: Empty) ; Poll :: Ready (res) } ThenProj :: Empty => panic ! ("ActorFuture polled after finish") , } } }
};
}
