macro_rules! deps {
    () => {
        IntegerType!();
        Endianness!();
        DecodeError!();
        Reader!();
    };
}

macro_rules! varint_decode_u16 {
    () => {
        deps!();
        pub fn varint_decode_u16 < R : Reader > (read : & mut R , endian : Endianness) -> Result < u16 , DecodeError > { if let Some (bytes) = read . peek_read (3) { let (discriminant , bytes) = bytes . split_at (1) ; let (out , used) = match discriminant [0] { byte @ 0 ..= SINGLE_BYTE_MAX => (byte as u16 , 1) , U16_BYTE => { let val = match endian { Endianness :: Big => u16 :: from_be_bytes (bytes [.. 2] . try_into () . unwrap ()) , Endianness :: Little => u16 :: from_le_bytes (bytes [.. 2] . try_into () . unwrap ()) , } ; (val , 3) } U32_BYTE => return invalid_varint_discriminant (IntegerType :: U16 , IntegerType :: U32) , U64_BYTE => return invalid_varint_discriminant (IntegerType :: U16 , IntegerType :: U64) , U128_BYTE => return invalid_varint_discriminant (IntegerType :: U16 , IntegerType :: U128) , _ => return invalid_varint_discriminant (IntegerType :: U16 , IntegerType :: Reserved) , } ; read . consume (used) ; Ok (out) } else { deserialize_varint_cold_u16 (read , endian) } }
    };
}

varint_decode_u16!()