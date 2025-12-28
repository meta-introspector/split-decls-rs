macro_rules! deps {
    () => {
        Endianness!();
        DecodeError!();
        Reader!();
    };
}

macro_rules! varint_decode_i64 {
    () => {
        deps!();
        pub fn varint_decode_i64 < R : Reader > (read : & mut R , endian : Endianness) -> Result < i64 , DecodeError > { let n = super :: varint_decode_u64 (read , endian) . map_err (DecodeError :: change_integer_type_to_signed) ? ; Ok (if n % 2 == 0 { (n / 2) as _ } else { ! (n / 2) as _ }) }
    };
}

varint_decode_i64!();