// Generated macro for exactly_one (function)
macro_rules! Depcrate_parse_utilsexactly_one {
() => {
// Module: crate::parse::utils
// Provides: {"exactly_one"}
// Dependencies: {}
pub (super) fn exactly_one < T > (iter : impl IntoIterator < Item = T >) -> T { let mut iter = iter . into_iter () ; let res = iter . next () . unwrap () ; debug_assert ! (iter . next () . is_none ()) ; res }
};
}
