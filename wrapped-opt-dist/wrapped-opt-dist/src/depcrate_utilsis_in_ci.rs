// Generated macro for is_in_ci (function)
macro_rules! Depcrate_utilsis_in_ci {
() => {
// Module: crate::utils
// Provides: {"is_in_ci"}
// Dependencies: {}
fn is_in_ci () -> bool { std :: env :: var ("GITHUB_ACTIONS") . is_ok () }
};
}
