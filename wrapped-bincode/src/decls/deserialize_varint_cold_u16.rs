macro_rules! deps {
    () => {
        Reader!();
        IntegerType!();
        DecodeError!();
        Endianness!();
    };
}

macro_rules! deserialize_varint_cold_u16 {
    () => {
        deps!();
        # [inline (never)] # [cold] fn deserialize_varint_cold_u16 < R > (read : & mut R , endian : Endianness) -> Result < u16 , DecodeError > where R : Reader , { let mut bytes = [0u8 ; 1] ; read . read (& mut bytes) ? ; match bytes [0] { byte @ 0 ..= SINGLE_BYTE_MAX => Ok (byte as u16) , U16_BYTE => { let mut bytes = [0u8 ; 2] ; read . read (& mut bytes) ? ; Ok (match endian { Endianness :: Big => u16 :: from_be_bytes (bytes) , Endianness :: Little => u16 :: from_le_bytes (bytes) , }) } U32_BYTE => invalid_varint_discriminant (IntegerType :: U16 , IntegerType :: U32) , U64_BYTE => invalid_varint_discriminant (IntegerType :: U16 , IntegerType :: U64) , U128_BYTE => invalid_varint_discriminant (IntegerType :: U16 , IntegerType :: U128) , _ => invalid_varint_discriminant (IntegerType :: U16 , IntegerType :: Reserved) , } }
    };
}

deserialize_varint_cold_u16!()