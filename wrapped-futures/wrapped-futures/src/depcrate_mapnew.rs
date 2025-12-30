// Generated macro for new (function)
macro_rules! Depcrate_mapnew {
() => {
// Module: crate::map
// Provides: {"new"}
// Dependencies: {}
pub fn new < A , F > (future : A , f : F) -> Map < A , F > where A : Future , { Map { future : Collapsed :: Start (future) , f : Some (f) , } }
};
}
