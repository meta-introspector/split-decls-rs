// Generated macro for C (function)
macro_rules! Depcrate_util_tC {
() => {
// Module: crate::util::t
// Provides: {"C"}
// Dependencies: {}
# [doc = " A short-hand creating a generic `Constant` value as a ranged integer."] # [doc = ""] # [doc = " Callers do need to ensure that the `MIN` and `MAX` bounds are specified (or"] # [doc = " more likely inferred), but otherwise, the `ri64` returned will be usable"] # [doc = " in most contexts even with other ranged integers (like `ri8`)."] # [allow (non_snake_case)] pub (crate) fn C (constant : i64 ,) -> ri64 < { i64 :: MIN as i128 } , { i64 :: MAX as i128 } > { Constant (constant) . rinto () }
};
}
