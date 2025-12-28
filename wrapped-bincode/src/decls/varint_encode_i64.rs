macro_rules! deps {
    () => {
        EncodeError!();
        Writer!();
        Endianness!();
    };
}

macro_rules! varint_encode_i64 {
    () => {
        deps!();
        pub fn varint_encode_i64 < W : Writer > (writer : & mut W , endian : Endianness , val : i64 ,) -> Result < () , EncodeError > { varint_encode_u64 (writer , endian , if val < 0 { ! (val as u64) * 2 + 1 } else { (val as u64) * 2 } ,) }
    };
}

varint_encode_i64!();