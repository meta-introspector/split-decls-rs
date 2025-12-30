// Generated macro for new (function)
macro_rules! Depcrate_thennew {
() => {
// Module: crate::then
// Provides: {"new"}
// Dependencies: {}
pub fn new < A , B , F > (future : A , f : F) -> Then < A , B , F > where A : Future , B : IntoFuture , F : Send + 'static , { Then { state : Chain :: new (future , f) , } }
};
}
