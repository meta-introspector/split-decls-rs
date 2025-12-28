macro_rules! deps {
    () => {
        DecodeError!();
        Endianness!();
        Reader!();
    };
}

macro_rules! varint_decode_i32 {
    () => {
        deps!();
        pub fn varint_decode_i32 < R : Reader > (read : & mut R , endian : Endianness) -> Result < i32 , DecodeError > { let n = super :: varint_decode_u32 (read , endian) . map_err (DecodeError :: change_integer_type_to_signed) ? ; Ok (if n % 2 == 0 { (n / 2) as _ } else { ! (n / 2) as _ }) }
    };
}

varint_decode_i32!();