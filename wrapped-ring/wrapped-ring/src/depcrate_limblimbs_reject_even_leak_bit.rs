// Generated macro for limbs_reject_even_leak_bit (function)
macro_rules! Depcrate_limblimbs_reject_even_leak_bit {
() => {
// Module: crate::limb
// Provides: {"limbs_reject_even_leak_bit"}
// Dependencies: {}
# [doc = " Leaks one bit of information (other than the lengths of the inputs):"] # [doc = " Whether the given limbs are even."] # [cfg (any (test , feature = "alloc"))] # [inline] pub fn limbs_reject_even_leak_bit (limbs : & [Limb]) -> Result < () , error :: Unspecified > { let bottom = * limbs . first () . ok_or (error :: Unspecified) ? ; if limb_is_zero (bottom & 1) . leak () { return Err (error :: Unspecified) ; } Ok (()) }
};
}
