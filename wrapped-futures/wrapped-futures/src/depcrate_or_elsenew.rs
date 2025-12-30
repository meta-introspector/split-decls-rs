// Generated macro for new (function)
macro_rules! Depcrate_or_elsenew {
() => {
// Module: crate::or_else
// Provides: {"new"}
// Dependencies: {}
pub fn new < A , B , F > (future : A , f : F) -> OrElse < A , B , F > where A : Future , B : IntoFuture < Item = A :: Item > , F : Send + 'static , { OrElse { state : Chain :: new (future , f) , } }
};
}
