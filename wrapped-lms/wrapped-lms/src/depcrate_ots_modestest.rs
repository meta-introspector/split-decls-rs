// Generated macro for test (module)
macro_rules! Depcrate_ots_modestest {
() => {
// Module: crate::ots::modes
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use hybrid_array :: Array ; use super :: LmsOtsMode ; # [test] fn test_checksum_zero_w1 () { let arr = [0u8 ; super :: LmsOtsSha256N32W1 :: N] ; let cksm = super :: LmsOtsSha256N32W1 :: expand (& Array :: from (arr)) ; assert_eq ! (& cksm [super :: LmsOtsSha256N32W1 :: U ..] , & [1 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0]) ; } # [test] fn test_checksum_ones_w1 () { let arr = [255u8 ; super :: LmsOtsSha256N32W1 :: N] ; let cksm = super :: LmsOtsSha256N32W1 :: expand (& Array :: from (arr)) ; assert_eq ! (& cksm [super :: LmsOtsSha256N32W1 :: U ..] , & [0 , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 0]) ; } # [test] fn test_checksum_ten_w4 () { let arr = [0xaa ; super :: LmsOtsSha256N32W4 :: N] ; let cksm = super :: LmsOtsSha256N32W4 :: expand (& Array :: from (arr)) ; assert_eq ! (& cksm [super :: LmsOtsSha256N32W4 :: U ..] , & [0x01 , 0x04 , 0x00]) ; } # [test] fn test_expand_zero_w8 () { let arr = [0u8 ; super :: LmsOtsSha256N32W8 :: N] ; let expanded = super :: LmsOtsSha256N32W8 :: expand (& Array :: from (arr)) ; let mut expected = [0u8 ; super :: LmsOtsSha256N32W8 :: P] ; expected [super :: LmsOtsSha256N32W8 :: U] = 0x1f ; expected [super :: LmsOtsSha256N32W8 :: U + 1] = 0xe0 ; assert_eq ! (& expanded . as_slice () , & expected) ; } }
};
}
