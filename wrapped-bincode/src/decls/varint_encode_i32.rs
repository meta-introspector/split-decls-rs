macro_rules! deps {
    () => {
        EncodeError!();
        Endianness!();
        Writer!();
    };
}

macro_rules! varint_encode_i32 {
    () => {
        deps!();
        pub fn varint_encode_i32 < W : Writer > (writer : & mut W , endian : Endianness , val : i32 ,) -> Result < () , EncodeError > { varint_encode_u32 (writer , endian , if val < 0 { ! (val as u32) * 2 + 1 } else { (val as u32) * 2 } ,) }
    };
}

varint_encode_i32!()