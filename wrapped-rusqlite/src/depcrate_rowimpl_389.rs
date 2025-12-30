// Generated macro for impl_389 (impl)
macro_rules! Depcrate_rowimpl_389 {
() => {
// Module: crate::row
// Provides: {"impl_389"}
// Dependencies: {}
impl < T , F > Iterator for MappedRows < '_ , F > where F : FnMut (& Row < '_ >) -> Result < T > , { type Item = Result < T > ; # [inline] fn next (& mut self) -> Option < Result < T > > { let map = & mut self . map ; self . rows . next () . transpose () . map (| row_result | row_result . and_then (map)) } }
};
}
