// Generated macro for impl_391 (impl)
macro_rules! Depcrate_rowimpl_391 {
() => {
// Module: crate::row
// Provides: {"impl_391"}
// Dependencies: {}
impl < T , E , F > Iterator for AndThenRows < '_ , F > where E : From < Error > , F : FnMut (& Row < '_ >) -> Result < T , E > , { type Item = Result < T , E > ; # [inline] fn next (& mut self) -> Option < Self :: Item > { let map = & mut self . map ; self . rows . next () . transpose () . map (| row_result | row_result . map_err (E :: from) . and_then (map)) } }
};
}
