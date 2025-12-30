// Generated macro for impl_2722 (impl)
macro_rules! Depcrate_loop_analysisimpl_2722 {
() => {
// Module: crate::loop_analysis
// Provides: {"impl_2722"}
// Dependencies: {}
impl LoopData { # [doc = " Creates a `LoopData` object with the loop header and its eventual parent in the loop tree."] pub fn new (header : Block , parent : Option < Loop >) -> Self { Self { header , parent : parent . into () , level : LoopLevel :: invalid () , } } }
};
}
