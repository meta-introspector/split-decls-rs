macro_rules! deps {
    () => {
        Endianness!();
        EncodeError!();
        Writer!();
    };
}

macro_rules! varint_encode_i128 {
    () => {
        deps!();
        pub fn varint_encode_i128 < W : Writer > (writer : & mut W , endian : Endianness , val : i128 ,) -> Result < () , EncodeError > { varint_encode_u128 (writer , endian , if val < 0 { ! (val as u128) * 2 + 1 } else { (val as u128) * 2 } ,) }
    };
}

varint_encode_i128!()