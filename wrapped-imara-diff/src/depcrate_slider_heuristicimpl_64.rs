// Generated macro for impl_64 (impl)
macro_rules! Depcrate_slider_heuristicimpl_64 {
() => {
// Module: crate::slider_heuristic
// Provides: {"impl_64"}
// Dependencies: {}
impl < F > SliderHeuristic for F where F : FnMut (& [Token] , Range < u32 > , u32) -> u32 , { fn best_slider_end (& mut self , tokens : & [Token] , hunk : Range < u32 > , earliest_end : u32) -> u32 { self (tokens , hunk , earliest_end) } }
};
}
