// Generated macro for test_base64_mising_padding (function)
macro_rules! Depcrate_base64test_base64_mising_padding {
() => {
// Module: crate::base64
// Provides: {"test_base64_mising_padding"}
// Dependencies: {}
# [cfg (feature = "std")] # [test] fn test_base64_mising_padding () { let missing_padding = "AA" ; assert ! (Base64 :: decode_to_vec (missing_padding , None) . is_err ()) ; assert ! (Base64NoPadding :: decode_to_vec (missing_padding , None) . is_ok ()) ; let missing_padding = "AAA" ; assert ! (Base64 :: decode_to_vec (missing_padding , None) . is_err ()) ; assert ! (Base64NoPadding :: decode_to_vec (missing_padding , None) . is_ok ()) ; }
};
}
