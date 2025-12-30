// Generated macro for test_decode_u16 (function)
macro_rules! Depcrate_varint_decode_unsignedtest_decode_u16 {
() => {
// Module: crate::varint::decode_unsigned
// Provides: {"test_decode_u16"}
// Dependencies: {}
# [test] fn test_decode_u16 () { let cases : & [(& [u8] , u16 , u16)] = & [(& [0] , 0 , 0) , (& [10] , 10 , 10) , (& [U16_BYTE , 0 , 10] , 2560 , 10) ,] ; for & (slice , expected_le , expected_be) in cases { let mut reader = crate :: de :: read :: SliceReader :: new (slice) ; let found = varint_decode_u16 (& mut reader , Endianness :: Little) . unwrap () ; assert_eq ! (expected_le , found) ; let mut reader = crate :: de :: read :: SliceReader :: new (slice) ; let found = varint_decode_u16 (& mut reader , Endianness :: Big) . unwrap () ; assert_eq ! (expected_be , found) ; } let errors : & [(& [u8] , DecodeError)] = & [(& [U32_BYTE] , DecodeError :: InvalidIntegerType { expected : IntegerType :: U16 , found : IntegerType :: U32 , } ,) , (& [U64_BYTE] , DecodeError :: InvalidIntegerType { expected : IntegerType :: U16 , found : IntegerType :: U64 , } ,) , (& [U128_BYTE] , DecodeError :: InvalidIntegerType { expected : IntegerType :: U16 , found : IntegerType :: U128 , } ,) , (& [U16_BYTE] , DecodeError :: UnexpectedEnd { additional : 2 }) , (& [U16_BYTE , 0] , DecodeError :: UnexpectedEnd { additional : 1 }) ,] ; for (slice , expected) in errors { let mut reader = crate :: de :: read :: SliceReader :: new (slice) ; let found = varint_decode_u16 (& mut reader , Endianness :: Little) . unwrap_err () ; assert_eq ! (std :: format ! ("{:?}" , expected) , std :: format ! ("{:?}" , found)) ; } }
};
}
