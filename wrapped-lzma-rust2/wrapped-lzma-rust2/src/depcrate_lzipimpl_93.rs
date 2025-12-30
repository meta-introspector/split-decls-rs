// Generated macro for impl_93 (impl)
macro_rules! Depcrate_lzipimpl_93 {
() => {
// Module: crate::lzip
// Provides: {"impl_93"}
// Dependencies: {}
impl LzipTrailer { fn parse < R : Read > (reader : & mut R) -> Result < Self > { let crc32 = reader . read_u32 () ? ; let data_size = reader . read_u64 () ? ; let member_size = reader . read_u64 () ? ; Ok (LzipTrailer { crc32 , data_size , member_size , }) } }
};
}
