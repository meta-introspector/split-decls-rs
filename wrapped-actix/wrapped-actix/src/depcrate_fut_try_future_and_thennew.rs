// Generated macro for new (function)
macro_rules! Depcrate_fut_try_future_and_thennew {
() => {
// Module: crate::fut::try_future::and_then
// Provides: {"new"}
// Dependencies: {}
pub (super) fn new < A , B , F , Act > (future : A , f : F) -> AndThen < A , B , F > where A : ActorTryFuture < Act > , B : ActorTryFuture < Act > , Act : Actor , { AndThen :: First { fut1 : future , data : Some (f) , } }
};
}
