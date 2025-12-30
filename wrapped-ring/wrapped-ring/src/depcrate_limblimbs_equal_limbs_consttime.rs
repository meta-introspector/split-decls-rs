// Generated macro for limbs_equal_limbs_consttime (function)
macro_rules! Depcrate_limblimbs_equal_limbs_consttime {
() => {
// Module: crate::limb
// Provides: {"limbs_equal_limbs_consttime"}
// Dependencies: {}
# [inline] pub fn limbs_equal_limbs_consttime (a : & [Limb] , b : & [Limb]) -> Result < LimbMask , LenMismatchError > { if a . len () != b . len () { return Err (LenMismatchError :: new (a . len ())) ; } let all = a . iter () . zip (b) . fold (0 , | running , (a , b) | running | (a ^ b)) ; Ok (limb_is_zero (all)) }
};
}
