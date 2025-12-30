// Generated macro for impl_89 (impl)
macro_rules! Depcrate_slider_heuristicimpl_89 {
() => {
// Module: crate::slider_heuristic
// Provides: {"impl_89"}
// Dependencies: {}
impl Score { fn is_improvement_over (self , prev_score : Self) -> bool { let indent_score = match prev_score . indent . cmp (& self . indent) { Ordering :: Less => INDENT_WEIGHT , Ordering :: Greater => - INDENT_WEIGHT , Ordering :: Equal => 0 , } ; (indent_score + self . penalty - prev_score . penalty) <= 0 } }
};
}
