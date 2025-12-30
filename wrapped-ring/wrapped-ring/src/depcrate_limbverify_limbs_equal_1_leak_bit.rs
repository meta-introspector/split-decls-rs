// Generated macro for verify_limbs_equal_1_leak_bit (function)
macro_rules! Depcrate_limbverify_limbs_equal_1_leak_bit {
() => {
// Module: crate::limb
// Provides: {"verify_limbs_equal_1_leak_bit"}
// Dependencies: {}
# [cfg (any (test , feature = "alloc"))] # [inline] pub fn verify_limbs_equal_1_leak_bit (a : & [Limb]) -> Result < () , error :: Unspecified > { if let [bottom , ref rest @ ..] = * a { let equal = limb_is_zero (bottom ^ 1) & limbs_are_zero (rest) ; if equal . leak () { return Ok (()) ; } } Err (error :: Unspecified) }
};
}
