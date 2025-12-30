// Generated macro for update_adler32 (function)
macro_rules! Depcrate_sharedupdate_adler32 {
() => {
// Module: crate::shared
// Provides: {"update_adler32"}
// Dependencies: {}
# [doc (hidden)] # [cfg (feature = "simd")] pub fn update_adler32 (adler : u32 , data : & [u8]) -> u32 { let mut hash = simd_adler32 :: Adler32 :: from_checksum (adler) ; hash . write (data) ; hash . finish () }
};
}
