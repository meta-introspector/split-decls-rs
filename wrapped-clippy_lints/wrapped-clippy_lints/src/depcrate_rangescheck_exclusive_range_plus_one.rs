// Generated macro for check_exclusive_range_plus_one (function)
macro_rules! Depcrate_rangescheck_exclusive_range_plus_one {
() => {
// Module: crate::ranges
// Provides: {"check_exclusive_range_plus_one"}
// Dependencies: {}
fn check_exclusive_range_plus_one < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { check_range_switch (cx , expr , RangeLimits :: HalfOpen , y_plus_one , RANGE_PLUS_ONE , "an inclusive range would be more readable" , "..=" ,) ; }
};
}
