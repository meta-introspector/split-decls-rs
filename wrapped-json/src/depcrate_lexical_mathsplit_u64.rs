// Generated macro for split_u64 (function)
macro_rules! Depcrate_lexical_mathsplit_u64 {
() => {
// Module: crate::lexical::math
// Provides: {"split_u64"}
// Dependencies: {}
# [doc = " Split u64 into limbs, in little-endian order."] # [inline] # [cfg (fast_arithmetic = "64")] fn split_u64 (x : u64) -> [Limb ; 1] { [as_limb (x)] }
};
}
