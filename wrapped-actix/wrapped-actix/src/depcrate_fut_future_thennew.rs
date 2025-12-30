// Generated macro for new (function)
macro_rules! Depcrate_fut_future_thennew {
() => {
// Module: crate::fut::future::then
// Provides: {"new"}
// Dependencies: {}
pub (super) fn new < A , B , F , Act > (future : A , f : F) -> Then < A , B , F > where A : ActorFuture < Act > , B : ActorFuture < Act > , Act : Actor , { Then :: First { fut1 : future , data : Some (f) , } }
};
}
