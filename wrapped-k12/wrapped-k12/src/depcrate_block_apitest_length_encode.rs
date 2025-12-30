// Generated macro for test_length_encode (function)
macro_rules! Depcrate_block_apitest_length_encode {
() => {
// Module: crate::block_api
// Provides: {"test_length_encode"}
// Dependencies: {}
# [test] fn test_length_encode () { let mut buffer = [0u8 ; LENGTH_ENCODE_SIZE] ; assert_eq ! (length_encode (0 , & mut buffer) , & [0x00]) ; assert_eq ! (length_encode (12 , & mut buffer) , & [0x0C , 0x01]) ; assert_eq ! (length_encode (65538 , & mut buffer) , & [0x01 , 0x00 , 0x02 , 0x03]) ; }
};
}
