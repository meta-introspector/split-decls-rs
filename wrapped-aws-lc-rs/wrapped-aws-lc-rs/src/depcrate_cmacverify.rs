// Generated macro for verify (function)
macro_rules! Depcrate_cmacverify {
() => {
// Module: crate::cmac
// Provides: {"verify"}
// Dependencies: {}
# [doc = " Calculates the CMAC of `data` using the signing key `key`, and verifies"] # [doc = " whether the resultant value equals `tag`, in one step."] # [doc = ""] # [doc = " The verification will be done in constant time to prevent timing attacks."] # [doc = ""] # [doc = " # Errors"] # [doc = " `error::Unspecified` if the inputs are not verified or CMAC calculation fails."] # [inline] pub fn verify (key : & Key , data : & [u8] , tag : & [u8]) -> Result < () , Unspecified > { let computed_tag = sign (key , data) ? ; constant_time :: verify_slices_are_equal (computed_tag . as_ref () , tag) }
};
}
