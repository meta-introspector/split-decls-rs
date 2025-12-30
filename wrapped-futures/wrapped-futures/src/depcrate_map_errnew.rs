// Generated macro for new (function)
macro_rules! Depcrate_map_errnew {
() => {
// Module: crate::map_err
// Provides: {"new"}
// Dependencies: {}
pub fn new < A , F > (future : A , f : F) -> MapErr < A , F > where A : Future { MapErr { future : Collapsed :: Start (future) , f : Some (f) , } }
};
}
