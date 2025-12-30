// Generated macro for test_base64_invalid_padding (function)
macro_rules! Depcrate_base64test_base64_invalid_padding {
() => {
// Module: crate::base64
// Provides: {"test_base64_invalid_padding"}
// Dependencies: {}
# [test] fn test_base64_invalid_padding () { let valid_padding = "AA==" ; assert_eq ! (Base64 :: decode_to_vec (valid_padding , None) , Ok (vec ! [0u8 ; 1])) ; let invalid_padding = "AA=" ; assert_eq ! (Base64 :: decode_to_vec (invalid_padding , None) , Err (Error :: InvalidInput)) ; }
};
}
