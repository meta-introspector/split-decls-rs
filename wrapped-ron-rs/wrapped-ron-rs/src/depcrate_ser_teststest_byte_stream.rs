// Generated macro for test_byte_stream (function)
macro_rules! Depcrate_ser_teststest_byte_stream {
() => {
// Module: crate::ser::tests
// Provides: {"test_byte_stream"}
// Dependencies: {}
# [test] fn test_byte_stream () { use serde_bytes ; let small : [u8 ; 16] = [0 , 1 , 2 , 3 , 4 , 5 , 6 , 7 , 8 , 9 , 10 , 11 , 12 , 13 , 14 , 15] ; check_to_string_writer (& small , "(0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15)" , "(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15)" ,) ; let large = vec ! [0x01 , 0x02 , 0x03 , 0x04] ; let large = serde_bytes :: Bytes :: new (& large) ; check_to_string_writer (& large , "b\"\\x01\\x02\\x03\\x04\"" , "b\"\\x01\\x02\\x03\\x04\"" ,) ; let large = vec ! [0x01 , 0x02 , 0x03 , 0x04 , 0x05 , 0x06] ; let large = serde_bytes :: Bytes :: new (& large) ; check_to_string_writer (& large , "b\"\\x01\\x02\\x03\\x04\\x05\\x06\"" , "b\"\\x01\\x02\\x03\\x04\\x05\\x06\"" ,) ; let large = vec ! [255u8 ; 64] ; let large = serde_bytes :: Bytes :: new (& large) ; check_to_string_writer (& large , concat ! ("b\"\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff" , "\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff" , "\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff" , "\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff" , "\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\"") , concat ! ("b\"\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff" , "\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff" , "\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff" , "\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff" , "\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\\xff\"") ,) ; }
};
}
