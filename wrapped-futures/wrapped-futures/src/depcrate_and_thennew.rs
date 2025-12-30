// Generated macro for new (function)
macro_rules! Depcrate_and_thennew {
() => {
// Module: crate::and_then
// Provides: {"new"}
// Dependencies: {}
pub fn new < A , B , F > (future : A , f : F) -> AndThen < A , B , F > where A : Future , B : IntoFuture , F : Send + 'static , { AndThen { state : Chain :: new (future , f) , } }
};
}
