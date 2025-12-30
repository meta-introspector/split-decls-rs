// Generated macro for test_base64_no_std (function)
macro_rules! Depcrate_base64test_base64_no_std {
() => {
// Module: crate::base64
// Provides: {"test_base64_no_std"}
// Dependencies: {}
# [test] fn test_base64_no_std () { let bin = [1u8 , 5 , 11 , 15 , 19 , 131 , 122] ; let expected = [65 , 81 , 85 , 76 , 68 , 120 , 79 , 68 , 101 , 103 , 61 , 61] ; let mut b64 = [0u8 ; 12] ; let b64 = Base64 :: encode (& mut b64 , bin) . unwrap () ; assert_eq ! (b64 , expected) ; let mut bin2 = [0u8 ; 7] ; let bin2 = Base64 :: decode (& mut bin2 , b64 , None) . unwrap () ; assert_eq ! (bin , bin2) ; }
};
}
