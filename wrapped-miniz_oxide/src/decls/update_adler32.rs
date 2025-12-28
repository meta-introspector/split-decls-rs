macro_rules! update_adler32 {
    () => {
        # [doc (hidden)] # [cfg (feature = "simd")] pub fn update_adler32 (adler : u32 , data : & [u8]) -> u32 { let mut hash = simd_adler32 :: Adler32 :: from_checksum (adler) ; hash . write (data) ; hash . finish () }
    };
}

update_adler32!();