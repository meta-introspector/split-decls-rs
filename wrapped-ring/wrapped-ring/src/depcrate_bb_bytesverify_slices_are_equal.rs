// Generated macro for verify_slices_are_equal (function)
macro_rules! Depcrate_bb_bytesverify_slices_are_equal {
() => {
// Module: crate::bb::bytes
// Provides: {"verify_slices_are_equal"}
// Dependencies: {}
# [doc = " Returns `Ok(())` if `a == b` and `Err(error::Unspecified)` otherwise."] pub fn verify_slices_are_equal (a : & [u8] , b : & [u8]) -> Result < () , error :: Unspecified > { if bytes_are_equal (a , b) . leak () { Ok (()) } else { Err (error :: Unspecified) } }
};
}
