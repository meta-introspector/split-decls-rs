// Generated macro for new (function)
macro_rules! Depcrate_fusenew {
() => {
// Module: crate::fuse
// Provides: {"new"}
// Dependencies: {}
pub fn new < A : Future > (f : A) -> Fuse < A > { Fuse { future : Some (f) , } }
};
}
