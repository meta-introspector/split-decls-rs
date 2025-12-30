// Generated macro for verify_slices_are_equal (function)
macro_rules! Depcrate_constant_timeverify_slices_are_equal {
() => {
// Module: crate::constant_time
// Provides: {"verify_slices_are_equal"}
// Dependencies: {}
# [doc = " Returns `Ok(())` if `a == b` and `Err(error::Unspecified)` otherwise."] # [doc = ""] # [doc = " The comparison of `a` and `b` is done in constant time with respect to the"] # [doc = " contents of each, but NOT in constant time with respect to the lengths of"] # [doc = " `a` and `b`."] # [doc = ""] # [doc = " # Errors"] # [doc = " `error::Unspecified` when `a` and `b` differ."] # [inline] pub fn verify_slices_are_equal (a : & [u8] , b : & [u8]) -> Result < () , error :: Unspecified > { if a . len () != b . len () { return Err (error :: Unspecified) ; } let result = unsafe { CRYPTO_memcmp (a . as_ptr () . cast () , b . as_ptr () . cast () , a . len ()) } ; match result { 0 => Ok (()) , _ => Err (error :: Unspecified) , } }
};
}
