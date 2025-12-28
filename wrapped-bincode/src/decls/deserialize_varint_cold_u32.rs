macro_rules! deps {
    () => {
        Endianness!();
        Reader!();
        IntegerType!();
        DecodeError!();
    };
}

macro_rules! deserialize_varint_cold_u32 {
    () => {
        deps!();
        # [inline (never)] # [cold] fn deserialize_varint_cold_u32 < R > (read : & mut R , endian : Endianness) -> Result < u32 , DecodeError > where R : Reader , { let mut bytes = [0u8 ; 1] ; read . read (& mut bytes) ? ; match bytes [0] { byte @ 0 ..= SINGLE_BYTE_MAX => Ok (byte as u32) , U16_BYTE => { let mut bytes = [0u8 ; 2] ; read . read (& mut bytes) ? ; Ok (match endian { Endianness :: Big => u16 :: from_be_bytes (bytes) as u32 , Endianness :: Little => u16 :: from_le_bytes (bytes) as u32 , }) } U32_BYTE => { let mut bytes = [0u8 ; 4] ; read . read (& mut bytes) ? ; Ok (match endian { Endianness :: Big => u32 :: from_be_bytes (bytes) , Endianness :: Little => u32 :: from_le_bytes (bytes) , }) } U64_BYTE => invalid_varint_discriminant (IntegerType :: U32 , IntegerType :: U64) , U128_BYTE => invalid_varint_discriminant (IntegerType :: U32 , IntegerType :: U128) , _ => invalid_varint_discriminant (IntegerType :: U32 , IntegerType :: Reserved) , } }
    };
}

deserialize_varint_cold_u32!();