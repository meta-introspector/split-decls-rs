macro_rules! deps {
    () => {
        SliceWriter!();
        Endianness!();
    };
}

macro_rules! test_encode_u16 {
    () => {
        deps!();
        # [test] fn test_encode_u16 () { use crate :: enc :: write :: SliceWriter ; let mut buffer = [0u8 ; 20] ; for i in 0u16 ..= SINGLE_BYTE_MAX as u16 { let mut writer = SliceWriter :: new (& mut buffer) ; varint_encode_u16 (& mut writer , Endianness :: Big , i) . unwrap () ; assert_eq ! (writer . bytes_written () , 1) ; assert_eq ! (buffer [0] as u16 , i) ; let mut writer = SliceWriter :: new (& mut buffer) ; varint_encode_u16 (& mut writer , Endianness :: Little , i) . unwrap () ; assert_eq ! (writer . bytes_written () , 1) ; assert_eq ! (buffer [0] as u16 , i) ; } for i in [SINGLE_BYTE_MAX as u16 + 1 , 300 , 500 , 700 , 888 , 1234 , u16 :: MAX ,] { let mut writer = SliceWriter :: new (& mut buffer) ; varint_encode_u16 (& mut writer , Endianness :: Big , i) . unwrap () ; assert_eq ! (writer . bytes_written () , 3) ; assert_eq ! (buffer [0] , U16_BYTE) ; assert_eq ! (& buffer [1 .. 3] , & i . to_be_bytes ()) ; let mut writer = SliceWriter :: new (& mut buffer) ; varint_encode_u16 (& mut writer , Endianness :: Little , i) . unwrap () ; assert_eq ! (writer . bytes_written () , 3) ; assert_eq ! (buffer [0] , U16_BYTE) ; assert_eq ! (& buffer [1 .. 3] , & i . to_le_bytes ()) ; } }
    };
}

test_encode_u16!();