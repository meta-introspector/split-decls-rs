// Generated macro for NoUnits (type)
macro_rules! Depcrate_util_tNoUnits {
() => {
// Module: crate::util::t
// Provides: {"NoUnits"}
// Dependencies: {}
# [doc = " A type alias for a ranged integer with no units."] # [doc = ""] # [doc = " In particular, the range of this type is just the range of an `i64`. This"] # [doc = " is useful when too many things with different units need to be combined at"] # [doc = " once, and it's just too painful to keep them straight. In cases like that,"] # [doc = " it's useful to just convert everything to `NoUnits`, do the necessary math,"] # [doc = " and then convert back to the appropriate ranged types."] # [doc = ""] # [doc = " Note that we don't actually lose much by doing this, since the computed"] # [doc = " min/max values are retained even when converting *to and from* this type."] # [doc = " In general, this type is just about making some math easier by making"] # [doc = " everything uniform."] pub (crate) type NoUnits = ri64 < { i64 :: MIN as i128 } , { i64 :: MAX as i128 } > ;
};
}
