// Generated macro for NoUnits128 (type)
macro_rules! Depcrate_util_tNoUnits128 {
() => {
// Module: crate::util::t
// Provides: {"NoUnits128"}
// Dependencies: {}
# [doc = " A type alias for a ranged 128-bit integer with no units."] # [doc = ""] # [doc = " This is like `NoUnits`, but useful in contexts where one wants to limit"] # [doc = " values to what can be represented by an `i128`."] pub (crate) type NoUnits128 = ri128 < { i128 :: MIN } , { i128 :: MAX } > ;
};
}
