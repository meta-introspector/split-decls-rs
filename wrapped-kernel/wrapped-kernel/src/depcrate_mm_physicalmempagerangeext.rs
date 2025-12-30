// Generated macro for PageRangeExt (trait)
macro_rules! Depcrate_mm_physicalmemPageRangeExt {
() => {
// Module: crate::mm::physicalmem
// Provides: {"PageRangeExt"}
// Dependencies: {}
trait PageRangeExt : Sized { fn containing (start : usize , end : usize) -> Result < Self , PageRangeError > ; fn and (self , rhs : Self) -> Option < Self > ; }
};
}
