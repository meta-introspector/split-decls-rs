// Generated macro for is_nightly (function)
macro_rules! Depcrateis_nightly {
() => {
// Module: crate
// Provides: {"is_nightly"}
// Dependencies: {}
pub fn is_nightly () -> bool { let vv = & rustc_info () . verbose_version ; env :: var ("CARGO_TEST_DISABLE_NIGHTLY") . is_err () && (vv . contains ("-nightly") || vv . contains ("-dev")) }
};
}
