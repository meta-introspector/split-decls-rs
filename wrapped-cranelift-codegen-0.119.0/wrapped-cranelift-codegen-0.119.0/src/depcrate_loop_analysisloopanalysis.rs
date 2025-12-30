// Generated macro for LoopAnalysis (struct)
macro_rules! Depcrate_loop_analysisLoopAnalysis {
() => {
// Module: crate::loop_analysis
// Provides: {"LoopAnalysis"}
// Dependencies: {}
# [doc = " Loop tree information for a single function."] # [doc = ""] # [doc = " Loops are referenced by the Loop object, and for each loop you can access its header block,"] # [doc = " its eventual parent in the loop tree and all the block belonging to the loop."] pub struct LoopAnalysis { loops : PrimaryMap < Loop , LoopData > , block_loop_map : SecondaryMap < Block , PackedOption < Loop > > , valid : bool , }
};
}
