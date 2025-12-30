// Generated macro for test (module)
macro_rules! Depcrate_coord_ranged1d_combinators_logarithmictest {
() => {
// Module: crate::coord::ranged1d::combinators::logarithmic
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn regression_test_issue_143 () { let range : LogCoord < f64 > = (1.0 .. 5.0) . log_scale () . into () ; range . key_points (100) ; } }
};
}
