// Generated macro for next_multiple_of (function)
macro_rules! Depcrate_util_tnext_multiple_of {
() => {
// Module: crate::util::t
// Provides: {"next_multiple_of"}
// Dependencies: {}
# [doc = " Computes the next multiple of `rhs` that is greater than or equal to `lhs`."] # [doc = ""] # [doc = " Taken from:"] # [doc = " https://github.com/rust-lang/rust/blob/eff958c59e8c07ba0515e164b825c9001b242294/library/core/src/num/int_macros.rs"] const fn next_multiple_of (lhs : i128 , rhs : i128) -> i128 { if rhs == - 1 { return lhs ; } let r = lhs % rhs ; let m = if (r > 0 && rhs < 0) || (r < 0 && rhs > 0) { r + rhs } else { r } ; if m == 0 { lhs } else { lhs + (rhs - m) } }
};
}
