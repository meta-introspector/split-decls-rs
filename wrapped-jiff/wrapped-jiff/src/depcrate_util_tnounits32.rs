// Generated macro for NoUnits32 (type)
macro_rules! Depcrate_util_tNoUnits32 {
() => {
// Module: crate::util::t
// Provides: {"NoUnits32"}
// Dependencies: {}
# [doc = " A type alias for a ranged 32-bit integer with no units."] # [doc = ""] # [doc = " This is like `NoUnits`, but useful in contexts where one wants to limit"] # [doc = " values to what can be represented by an `i32`."] pub (crate) type NoUnits32 = ri32 < { i32 :: MIN as i128 } , { i32 :: MAX as i128 } > ;
};
}
