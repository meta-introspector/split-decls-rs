// Generated macro for verify_limbs_less_than_limbs_leak_bit (function)
macro_rules! Depcrate_limbverify_limbs_less_than_limbs_leak_bit {
() => {
// Module: crate::limb
// Provides: {"verify_limbs_less_than_limbs_leak_bit"}
// Dependencies: {}
# [inline] pub (crate) fn verify_limbs_less_than_limbs_leak_bit (a : & [Limb] , b : & [Limb] ,) -> Result < () , error :: Unspecified > { let r = limbs_less_than_limbs (a , b) . map_err (error :: erase :: < LenMismatchError >) ? ; if r . leak () { Ok (()) } else { Err (error :: Unspecified) } }
};
}
