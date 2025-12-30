// Generated macro for NoUnits16 (type)
macro_rules! Depcrate_util_tNoUnits16 {
() => {
// Module: crate::util::t
// Provides: {"NoUnits16"}
// Dependencies: {}
# [doc = " A type alias for a ranged 16-bit integer with no units."] # [doc = ""] # [doc = " This is like `NoUnits`, but useful in contexts where one wants to limit"] # [doc = " values to what can be represented by an `i16`."] pub (crate) type NoUnits16 = ri16 < { i16 :: MIN as i128 } , { i16 :: MAX as i128 } > ;
};
}
