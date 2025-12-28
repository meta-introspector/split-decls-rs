macro_rules! deps {
    () => {
        EncodeError!();
        Writer!();
        Endianness!();
    };
}

macro_rules! varint_encode_usize {
    () => {
        deps!();
        pub fn varint_encode_usize < W : Writer > (writer : & mut W , endian : Endianness , val : usize ,) -> Result < () , EncodeError > { varint_encode_u64 (writer , endian , val as u64) }
    };
}

varint_encode_usize!()