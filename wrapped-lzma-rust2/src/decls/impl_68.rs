macro_rules! deps {
    () => {
        Read!();
        LzipTrailer!();
        Result!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl LzipTrailer { fn parse < R : Read > (reader : & mut R) -> Result < Self > { let crc32 = reader . read_u32 () ? ; let data_size = reader . read_u64 () ? ; let member_size = reader . read_u64 () ? ; Ok (LzipTrailer { crc32 , data_size , member_size , }) } }
    };
}

impl_68!();