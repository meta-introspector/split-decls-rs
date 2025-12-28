macro_rules! deps {
    () => {
        DecodeError!();
        Endianness!();
        IntegerType!();
        Reader!();
    };
}

macro_rules! deserialize_varint_cold_u128 {
    () => {
        deps!();
        # [inline (never)] # [cold] fn deserialize_varint_cold_u128 < R > (read : & mut R , endian : Endianness) -> Result < u128 , DecodeError > where R : Reader , { let mut bytes = [0u8 ; 1] ; read . read (& mut bytes) ? ; match bytes [0] { byte @ 0 ..= SINGLE_BYTE_MAX => Ok (byte as u128) , U16_BYTE => { let mut bytes = [0u8 ; 2] ; read . read (& mut bytes) ? ; Ok (match endian { Endianness :: Big => u16 :: from_be_bytes (bytes) as u128 , Endianness :: Little => u16 :: from_le_bytes (bytes) as u128 , }) } U32_BYTE => { let mut bytes = [0u8 ; 4] ; read . read (& mut bytes) ? ; Ok (match endian { Endianness :: Big => u32 :: from_be_bytes (bytes) as u128 , Endianness :: Little => u32 :: from_le_bytes (bytes) as u128 , }) } U64_BYTE => { let mut bytes = [0u8 ; 8] ; read . read (& mut bytes) ? ; Ok (match endian { Endianness :: Big => u64 :: from_be_bytes (bytes) as u128 , Endianness :: Little => u64 :: from_le_bytes (bytes) as u128 , }) } U128_BYTE => { let mut bytes = [0u8 ; 16] ; read . read (& mut bytes) ? ; Ok (match endian { Endianness :: Big => u128 :: from_be_bytes (bytes) , Endianness :: Little => u128 :: from_le_bytes (bytes) , }) } _ => invalid_varint_discriminant (IntegerType :: U128 , IntegerType :: Reserved) , } }
    };
}

deserialize_varint_cold_u128!()