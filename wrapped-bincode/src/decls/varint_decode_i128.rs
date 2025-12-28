macro_rules! deps {
    () => {
        Reader!();
        Endianness!();
        DecodeError!();
    };
}

macro_rules! varint_decode_i128 {
    () => {
        deps!();
        pub fn varint_decode_i128 < R : Reader > (read : & mut R , endian : Endianness ,) -> Result < i128 , DecodeError > { let n = super :: varint_decode_u128 (read , endian) . map_err (DecodeError :: change_integer_type_to_signed) ? ; Ok (if n % 2 == 0 { (n / 2) as _ } else { ! (n / 2) as _ }) }
    };
}

varint_decode_i128!();