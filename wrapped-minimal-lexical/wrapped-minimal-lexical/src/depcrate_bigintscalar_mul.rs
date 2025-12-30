// Generated macro for scalar_mul (function)
macro_rules! Depcrate_bigintscalar_mul {
() => {
// Module: crate::bigint
// Provides: {"scalar_mul"}
// Dependencies: {}
# [doc = " Multiply two small integers (with carry) (and return the overflow contribution)."] # [doc = ""] # [doc = " Returns the (low, high) components."] # [inline (always)] pub fn scalar_mul (x : Limb , y : Limb , carry : Limb) -> (Limb , Limb) { let z : Wide = (x as Wide) * (y as Wide) + (carry as Wide) ; (z as Limb , (z >> LIMB_BITS) as Limb) }
};
}
