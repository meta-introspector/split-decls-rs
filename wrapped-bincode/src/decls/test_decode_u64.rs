macro_rules! deps {
    () => {
        SliceReader!();
        IntegerType!();
        DecodeError!();
        Endianness!();
    };
}

macro_rules! test_decode_u64 {
    () => {
        deps!();
        # [test] fn test_decode_u64 () { let cases : & [(& [u8] , u64 , u64)] = & [(& [0] , 0 , 0) , (& [10] , 10 , 10) , (& [U16_BYTE , 0 , 10] , 2560 , 10) , (& [U32_BYTE , 0 , 0 , 0 , 10] , 167_772_160 , 10) , (& [U64_BYTE , 0 , 0 , 0 , 0 , 0 , 0 , 0 , 10] , 720_575_940_379_279_360 , 10 ,) ,] ; for & (slice , expected_le , expected_be) in cases { let mut reader = crate :: de :: read :: SliceReader :: new (slice) ; let found = varint_decode_u64 (& mut reader , Endianness :: Little) . unwrap () ; assert_eq ! (expected_le , found) ; let mut reader = crate :: de :: read :: SliceReader :: new (slice) ; let found = varint_decode_u64 (& mut reader , Endianness :: Big) . unwrap () ; assert_eq ! (expected_be , found) ; } let errors : & [(& [u8] , DecodeError)] = & [(& [U128_BYTE] , DecodeError :: InvalidIntegerType { expected : IntegerType :: U64 , found : IntegerType :: U128 , } ,) , (& [U16_BYTE] , DecodeError :: UnexpectedEnd { additional : 2 }) , (& [U16_BYTE , 0] , DecodeError :: UnexpectedEnd { additional : 1 }) , (& [U32_BYTE] , DecodeError :: UnexpectedEnd { additional : 4 }) , (& [U32_BYTE , 0] , DecodeError :: UnexpectedEnd { additional : 3 }) , (& [U32_BYTE , 0 , 0] , DecodeError :: UnexpectedEnd { additional : 2 } ,) , (& [U32_BYTE , 0 , 0 , 0] , DecodeError :: UnexpectedEnd { additional : 1 } ,) , (& [U64_BYTE] , DecodeError :: UnexpectedEnd { additional : 8 }) , (& [U64_BYTE , 0] , DecodeError :: UnexpectedEnd { additional : 7 }) , (& [U64_BYTE , 0 , 0] , DecodeError :: UnexpectedEnd { additional : 6 } ,) , (& [U64_BYTE , 0 , 0 , 0] , DecodeError :: UnexpectedEnd { additional : 5 } ,) , (& [U64_BYTE , 0 , 0 , 0 , 0] , DecodeError :: UnexpectedEnd { additional : 4 } ,) , (& [U64_BYTE , 0 , 0 , 0 , 0 , 0] , DecodeError :: UnexpectedEnd { additional : 3 } ,) , (& [U64_BYTE , 0 , 0 , 0 , 0 , 0 , 0] , DecodeError :: UnexpectedEnd { additional : 2 } ,) , (& [U64_BYTE , 0 , 0 , 0 , 0 , 0 , 0 , 0] , DecodeError :: UnexpectedEnd { additional : 1 } ,) ,] ; for (slice , expected) in errors { let mut reader = crate :: de :: read :: SliceReader :: new (slice) ; let found = varint_decode_u64 (& mut reader , Endianness :: Little) . unwrap_err () ; assert_eq ! (std :: format ! ("{:?}" , expected) , std :: format ! ("{:?}" , found)) ; } }
    };
}

test_decode_u64!();