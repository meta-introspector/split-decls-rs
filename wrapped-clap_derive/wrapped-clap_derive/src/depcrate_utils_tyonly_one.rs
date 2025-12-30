// Generated macro for only_one (function)
macro_rules! Depcrate_utils_tyonly_one {
() => {
// Module: crate::utils::ty
// Provides: {"only_one"}
// Dependencies: {}
fn only_one < I , T > (mut iter : I) -> Option < T > where I : Iterator < Item = T > , { iter . next () . filter (| _ | iter . next () . is_none ()) }
};
}
