macro_rules! deps {
    () => {
        Endianness!();
        EncodeError!();
        Writer!();
    };
}

macro_rules! varint_encode_i16 {
    () => {
        deps!();
        pub fn varint_encode_i16 < W : Writer > (writer : & mut W , endian : Endianness , val : i16 ,) -> Result < () , EncodeError > { varint_encode_u16 (writer , endian , if val < 0 { ! (val as u16) * 2 + 1 } else { (val as u16) * 2 } ,) }
    };
}

varint_encode_i16!();