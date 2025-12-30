// Generated macro for needs_exact_match (function)
macro_rules! Depcrate_rewrites_trackerneeds_exact_match {
() => {
// Module: crate::rewrites::tracker
// Provides: {"needs_exact_match"}
// Dependencies: {}
fn needs_exact_match (percentage : Option < f32 >) -> bool { percentage . is_none_or (| p | p >= 1.0) }
};
}
