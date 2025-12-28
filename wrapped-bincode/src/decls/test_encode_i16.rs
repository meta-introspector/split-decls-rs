macro_rules! deps {
    () => {
        Endianness!();
        SliceWriter!();
    };
}

macro_rules! test_encode_i16 {
    () => {
        deps!();
        # [test] fn test_encode_i16 () { let cases : & [(i16 , & [u8] , & [u8])] = & [(0 , & [0] , & [0]) , (2 , & [4] , & [4]) , (256 , & [super :: U16_BYTE , 0 , 2] , & [super :: U16_BYTE , 2 , 0]) , (16_000 , & [super :: U16_BYTE , 0 , 125] , & [super :: U16_BYTE , 125 , 0] ,) , (i16 :: MAX - 1 , & [super :: U16_BYTE , 252 , 255] , & [super :: U16_BYTE , 255 , 252] ,) , (i16 :: MAX , & [super :: U16_BYTE , 254 , 255] , & [super :: U16_BYTE , 255 , 254] ,) ,] ; use crate :: enc :: write :: SliceWriter ; let mut buffer = [0u8 ; 20] ; for & (value , expected_le , expected_be) in cases { std :: dbg ! (value) ; let mut writer = SliceWriter :: new (& mut buffer) ; varint_encode_i16 (& mut writer , Endianness :: Little , value) . unwrap () ; assert_eq ! (writer . bytes_written () , expected_le . len ()) ; assert_eq ! (& buffer [.. expected_le . len ()] , expected_le) ; let mut writer = SliceWriter :: new (& mut buffer) ; varint_encode_i16 (& mut writer , Endianness :: Big , value) . unwrap () ; assert_eq ! (writer . bytes_written () , expected_be . len ()) ; assert_eq ! (& buffer [.. expected_be . len ()] , expected_be) ; } }
    };
}

test_encode_i16!();