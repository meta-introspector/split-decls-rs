macro_rules! deps {
    () => {
        SliceWriter!();
        Endianness!();
    };
}

macro_rules! test_encode_u64 {
    () => {
        deps!();
        # [test] fn test_encode_u64 () { use crate :: enc :: write :: SliceWriter ; let mut buffer = [0u8 ; 20] ; for i in 0u64 ..= SINGLE_BYTE_MAX as u64 { let mut writer = SliceWriter :: new (& mut buffer) ; varint_encode_u64 (& mut writer , Endianness :: Big , i) . unwrap () ; assert_eq ! (writer . bytes_written () , 1) ; assert_eq ! (buffer [0] as u64 , i) ; let mut writer = SliceWriter :: new (& mut buffer) ; varint_encode_u64 (& mut writer , Endianness :: Little , i) . unwrap () ; assert_eq ! (writer . bytes_written () , 1) ; assert_eq ! (buffer [0] as u64 , i) ; } for i in [SINGLE_BYTE_MAX as u64 + 1 , 300 , 500 , 700 , 888 , 1234 , u16 :: MAX as u64 ,] { let mut writer = SliceWriter :: new (& mut buffer) ; varint_encode_u64 (& mut writer , Endianness :: Big , i) . unwrap () ; assert_eq ! (writer . bytes_written () , 3) ; assert_eq ! (buffer [0] , U16_BYTE) ; assert_eq ! (& buffer [1 .. 3] , & (i as u16) . to_be_bytes ()) ; let mut writer = SliceWriter :: new (& mut buffer) ; varint_encode_u64 (& mut writer , Endianness :: Little , i) . unwrap () ; assert_eq ! (writer . bytes_written () , 3) ; assert_eq ! (buffer [0] , U16_BYTE) ; assert_eq ! (& buffer [1 .. 3] , & (i as u16) . to_le_bytes ()) ; } for i in [u16 :: MAX as u64 + 1 , 100_000 , 1_000_000 , u32 :: MAX as u64] { let mut writer = SliceWriter :: new (& mut buffer) ; varint_encode_u64 (& mut writer , Endianness :: Big , i) . unwrap () ; assert_eq ! (writer . bytes_written () , 5) ; assert_eq ! (buffer [0] , U32_BYTE) ; assert_eq ! (& buffer [1 .. 5] , & (i as u32) . to_be_bytes ()) ; let mut writer = SliceWriter :: new (& mut buffer) ; varint_encode_u64 (& mut writer , Endianness :: Little , i) . unwrap () ; assert_eq ! (writer . bytes_written () , 5) ; assert_eq ! (buffer [0] , U32_BYTE) ; assert_eq ! (& buffer [1 .. 5] , & (i as u32) . to_le_bytes ()) ; } for i in [u32 :: MAX as u64 + 1 , 5_000_000_000 , u64 :: MAX] { let mut writer = SliceWriter :: new (& mut buffer) ; varint_encode_u64 (& mut writer , Endianness :: Big , i) . unwrap () ; assert_eq ! (writer . bytes_written () , 9) ; assert_eq ! (buffer [0] , U64_BYTE) ; assert_eq ! (& buffer [1 .. 9] , & i . to_be_bytes ()) ; let mut writer = SliceWriter :: new (& mut buffer) ; varint_encode_u64 (& mut writer , Endianness :: Little , i) . unwrap () ; assert_eq ! (writer . bytes_written () , 9) ; assert_eq ! (buffer [0] , U64_BYTE) ; assert_eq ! (& buffer [1 .. 9] , & i . to_le_bytes ()) ; } }
    };
}

test_encode_u64!()