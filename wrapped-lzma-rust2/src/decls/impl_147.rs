macro_rules! deps {
    () => {
        CheckType!();
        Result!();
    };
}

macro_rules! impl_147 {
    () => {
        deps!();
        impl CheckType { fn from_byte (byte : u8) -> crate :: Result < Self > { match byte { 0x00 => Ok (CheckType :: None) , 0x01 => Ok (CheckType :: Crc32) , 0x04 => Ok (CheckType :: Crc64) , 0x0A => Ok (CheckType :: Sha256) , _ => Err (error_invalid_data ("unsupported XZ check type")) , } } # [cfg (any (feature = "encoder" , feature = "xz"))] fn checksum_size (self) -> u64 { match self { CheckType :: None => 0 , CheckType :: Crc32 => 4 , CheckType :: Crc64 => 8 , CheckType :: Sha256 => 32 , } } }
    };
}

impl_147!()