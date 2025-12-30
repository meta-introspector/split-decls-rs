// Generated macro for check_inclusive_range_minus_one (function)
macro_rules! Depcrate_rangescheck_inclusive_range_minus_one {
() => {
// Module: crate::ranges
// Provides: {"check_inclusive_range_minus_one"}
// Dependencies: {}
fn check_inclusive_range_minus_one < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { check_range_switch (cx , expr , RangeLimits :: Closed , y_minus_one , RANGE_MINUS_ONE , "an exclusive range would be more readable" , ".." ,) ; }
};
}
