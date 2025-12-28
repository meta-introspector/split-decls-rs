macro_rules! deps {
    () => {
        SliceWriter!();
        Endianness!();
    };
}

macro_rules! test_encode_i32 {
    () => {
        deps!();
        # [test] fn test_encode_i32 () { let cases : & [(i32 , & [u8] , & [u8])] = & [(0 , & [0] , & [0]) , (2 , & [4] , & [4]) , (256 , & [super :: U16_BYTE , 0 , 2] , & [super :: U16_BYTE , 2 , 0]) , (16_000 , & [super :: U16_BYTE , 0 , 125] , & [super :: U16_BYTE , 125 , 0] ,) , (40_000 , & [super :: U32_BYTE , 128 , 56 , 1 , 0] , & [super :: U32_BYTE , 0 , 1 , 56 , 128] ,) , (i32 :: MAX - 1 , & [super :: U32_BYTE , 252 , 255 , 255 , 255] , & [super :: U32_BYTE , 255 , 255 , 255 , 252] ,) , (i32 :: MAX , & [super :: U32_BYTE , 254 , 255 , 255 , 255] , & [super :: U32_BYTE , 255 , 255 , 255 , 254] ,) ,] ; use crate :: enc :: write :: SliceWriter ; let mut buffer = [0u8 ; 20] ; for & (value , expected_le , expected_be) in cases { std :: dbg ! (value) ; let mut writer = SliceWriter :: new (& mut buffer) ; varint_encode_i32 (& mut writer , Endianness :: Little , value) . unwrap () ; assert_eq ! (writer . bytes_written () , expected_le . len ()) ; assert_eq ! (& buffer [.. expected_le . len ()] , expected_le) ; let mut writer = SliceWriter :: new (& mut buffer) ; varint_encode_i32 (& mut writer , Endianness :: Big , value) . unwrap () ; assert_eq ! (writer . bytes_written () , expected_be . len ()) ; assert_eq ! (& buffer [.. expected_be . len ()] , expected_be) ; } }
    };
}

test_encode_i32!()