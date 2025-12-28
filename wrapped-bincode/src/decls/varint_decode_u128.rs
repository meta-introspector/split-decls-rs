macro_rules! deps {
    () => {
        DecodeError!();
        IntegerType!();
        Endianness!();
        Reader!();
    };
}

macro_rules! varint_decode_u128 {
    () => {
        deps!();
        pub fn varint_decode_u128 < R : Reader > (read : & mut R , endian : Endianness ,) -> Result < u128 , DecodeError > { if let Some (bytes) = read . peek_read (17) { let (discriminant , bytes) = bytes . split_at (1) ; let (out , used) = match discriminant [0] { byte @ 0 ..= SINGLE_BYTE_MAX => (byte as u128 , 1) , U16_BYTE => { let val = match endian { Endianness :: Big => u16 :: from_be_bytes (bytes [.. 2] . try_into () . unwrap ()) , Endianness :: Little => u16 :: from_le_bytes (bytes [.. 2] . try_into () . unwrap ()) , } ; (val as u128 , 3) } U32_BYTE => { let val = match endian { Endianness :: Big => u32 :: from_be_bytes (bytes [.. 4] . try_into () . unwrap ()) , Endianness :: Little => u32 :: from_le_bytes (bytes [.. 4] . try_into () . unwrap ()) , } ; (val as u128 , 5) } U64_BYTE => { let val = match endian { Endianness :: Big => u64 :: from_be_bytes (bytes [.. 8] . try_into () . unwrap ()) , Endianness :: Little => u64 :: from_le_bytes (bytes [.. 8] . try_into () . unwrap ()) , } ; (val as u128 , 9) } U128_BYTE => { let val = match endian { Endianness :: Big => u128 :: from_be_bytes (bytes [.. 16] . try_into () . unwrap ()) , Endianness :: Little => u128 :: from_le_bytes (bytes [.. 16] . try_into () . unwrap ()) , } ; (val , 17) } _ => return invalid_varint_discriminant (IntegerType :: Usize , IntegerType :: Reserved) , } ; read . consume (used) ; Ok (out) } else { deserialize_varint_cold_u128 (read , endian) } }
    };
}

varint_decode_u128!()