// Generated macro for compare_to_threshold (function)
macro_rules! Depcrate_htmlcompare_to_threshold {
() => {
// Module: crate::html
// Provides: {"compare_to_threshold"}
// Dependencies: {}
fn compare_to_threshold (estimate : & Estimate , noise : f64) -> ComparisonResult { let ci = & estimate . confidence_interval ; let lb = ci . lower_bound ; let ub = ci . upper_bound ; if lb < - noise && ub < - noise { ComparisonResult :: Improved } else if lb > noise && ub > noise { ComparisonResult :: Regressed } else { ComparisonResult :: NonSignificant } }
};
}
