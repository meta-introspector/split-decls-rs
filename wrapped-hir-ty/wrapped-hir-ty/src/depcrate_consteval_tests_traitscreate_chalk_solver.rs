// Generated macro for create_chalk_solver (function)
macro_rules! Depcrate_consteval_tests_traitscreate_chalk_solver {
() => {
// Module: crate::consteval::tests::traits
// Provides: {"create_chalk_solver"}
// Dependencies: {}
fn create_chalk_solver () -> chalk_recursive :: RecursiveSolver < Interner > { let overflow_depth = var ("CHALK_OVERFLOW_DEPTH") . ok () . and_then (| s | s . parse () . ok ()) . unwrap_or (500) ; let max_size = var ("CHALK_SOLVER_MAX_SIZE") . ok () . and_then (| s | s . parse () . ok ()) . unwrap_or (150) ; chalk_recursive :: RecursiveSolver :: new (overflow_depth , max_size , Some (Cache :: new ())) }
};
}
