macro_rules! deps {
    () => {
        DecodeError!();
        IntegerType!();
        Reader!();
        Endianness!();
    };
}

macro_rules! varint_decode_u64 {
    () => {
        deps!();
        pub fn varint_decode_u64 < R : Reader > (read : & mut R , endian : Endianness) -> Result < u64 , DecodeError > { if let Some (bytes) = read . peek_read (9) { let (discriminant , bytes) = bytes . split_at (1) ; let (out , used) = match discriminant [0] { byte @ 0 ..= SINGLE_BYTE_MAX => (byte as u64 , 1) , U16_BYTE => { let val = match endian { Endianness :: Big => u16 :: from_be_bytes (bytes [.. 2] . try_into () . unwrap ()) , Endianness :: Little => u16 :: from_le_bytes (bytes [.. 2] . try_into () . unwrap ()) , } ; (val as u64 , 3) } U32_BYTE => { let val = match endian { Endianness :: Big => u32 :: from_be_bytes (bytes [.. 4] . try_into () . unwrap ()) , Endianness :: Little => u32 :: from_le_bytes (bytes [.. 4] . try_into () . unwrap ()) , } ; (val as u64 , 5) } U64_BYTE => { let val = match endian { Endianness :: Big => u64 :: from_be_bytes (bytes [.. 8] . try_into () . unwrap ()) , Endianness :: Little => u64 :: from_le_bytes (bytes [.. 8] . try_into () . unwrap ()) , } ; (val , 9) } U128_BYTE => return invalid_varint_discriminant (IntegerType :: U32 , IntegerType :: U128) , _ => return invalid_varint_discriminant (IntegerType :: U32 , IntegerType :: Reserved) , } ; read . consume (used) ; Ok (out) } else { deserialize_varint_cold_u64 (read , endian) } }
    };
}

varint_decode_u64!();