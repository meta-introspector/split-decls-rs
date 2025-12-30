// Generated macro for impl_294 (impl)
macro_rules! Depcrate_fut_future_eitherimpl_294 {
() => {
// Module: crate::fut::future::either
// Provides: {"impl_294"}
// Dependencies: {}
impl < A , B , Act > ActorFuture < Act > for Either < A , B > where A : ActorFuture < Act > , B : ActorFuture < Act , Output = A :: Output > , Act : Actor , { type Output = A :: Output ; fn poll (self : Pin < & mut Self > , act : & mut Act , ctx : & mut Act :: Context , task : & mut Context < '_ > ,) -> Poll < A :: Output > { let this = unsafe { match self . get_unchecked_mut () { Either :: Left (a) => Either :: Left (Pin :: new_unchecked (a)) , Either :: Right (b) => Either :: Right (Pin :: new_unchecked (b)) , } } ; match this { Either :: Left (left) => left . poll (act , ctx , task) , Either :: Right (right) => right . poll (act , ctx , task) , } } }
};
}
