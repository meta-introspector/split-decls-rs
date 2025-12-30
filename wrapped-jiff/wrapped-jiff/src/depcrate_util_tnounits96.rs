// Generated macro for NoUnits96 (type)
macro_rules! Depcrate_util_tNoUnits96 {
() => {
// Module: crate::util::t
// Provides: {"NoUnits96"}
// Dependencies: {}
# [doc = " A type alias for a ranged 96-bit integer with no units."] # [doc = ""] # [doc = " This is like `NoUnits`, but useful in contexts where one wants to limit"] # [doc = " values to what can be represented to 96 bits."] pub (crate) type NoUnits96 = ri128 < { - (1 << 95) } , { (1 << 95) - 1 } > ;
};
}
