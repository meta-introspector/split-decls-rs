// Generated macro for new (function)
macro_rules! Depcrate_flattennew {
() => {
// Module: crate::flatten
// Provides: {"new"}
// Dependencies: {}
pub fn new < A > (future : A) -> Flatten < A > where A : Future , A :: Item : IntoFuture , { Flatten { state : Chain :: new (future , ()) , } }
};
}
