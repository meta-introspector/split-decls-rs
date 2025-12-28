macro_rules! deps {
    () => {
        Writer!();
        EncodeError!();
        Endianness!();
    };
}

macro_rules! varint_encode_isize {
    () => {
        deps!();
        pub fn varint_encode_isize < W : Writer > (writer : & mut W , endian : Endianness , val : isize ,) -> Result < () , EncodeError > { varint_encode_i64 (writer , endian , val as i64) }
    };
}

varint_encode_isize!();