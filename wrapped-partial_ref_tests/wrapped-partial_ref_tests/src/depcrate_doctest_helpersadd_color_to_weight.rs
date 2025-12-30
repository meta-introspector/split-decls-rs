// Generated macro for add_color_to_weight (function)
macro_rules! Depcrate_doctest_helpersadd_color_to_weight {
() => {
// Module: crate::doctest_helpers
// Provides: {"add_color_to_weight"}
// Dependencies: {}
pub fn add_color_to_weight (mut g : partial ! (Graph , mut Weights , Colors) , index : usize) { g . part_mut (Weights) [index] += g . part (Colors) [index] as f32 ; }
};
}
