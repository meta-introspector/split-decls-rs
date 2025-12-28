macro_rules! deps {
    () => {
        IntegerType!();
        DecodeError!();
        Endianness!();
        Reader!();
    };
}

macro_rules! deserialize_varint_cold_usize {
    () => {
        deps!();
        # [inline (never)] # [cold] fn deserialize_varint_cold_usize < R > (read : & mut R , endian : Endianness) -> Result < usize , DecodeError > where R : Reader , { let mut bytes = [0u8 ; 1] ; read . read (& mut bytes) ? ; match bytes [0] { byte @ 0 ..= SINGLE_BYTE_MAX => Ok (byte as usize) , U16_BYTE => { let mut bytes = [0u8 ; 2] ; read . read (& mut bytes) ? ; Ok (match endian { Endianness :: Big => u16 :: from_be_bytes (bytes) as usize , Endianness :: Little => u16 :: from_le_bytes (bytes) as usize , }) } U32_BYTE => { let mut bytes = [0u8 ; 4] ; read . read (& mut bytes) ? ; Ok (match endian { Endianness :: Big => u32 :: from_be_bytes (bytes) as usize , Endianness :: Little => u32 :: from_le_bytes (bytes) as usize , }) } U64_BYTE => { let mut bytes = [0u8 ; 8] ; read . read (& mut bytes) ? ; Ok (match endian { Endianness :: Big => u64 :: from_be_bytes (bytes) as usize , Endianness :: Little => u64 :: from_le_bytes (bytes) as usize , }) } U128_BYTE => invalid_varint_discriminant (IntegerType :: Usize , IntegerType :: U128) , _ => invalid_varint_discriminant (IntegerType :: Usize , IntegerType :: Reserved) , } }
    };
}

deserialize_varint_cold_usize!()