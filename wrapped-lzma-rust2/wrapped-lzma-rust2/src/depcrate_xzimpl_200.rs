// Generated macro for impl_200 (impl)
macro_rules! Depcrate_xzimpl_200 {
() => {
// Module: crate::xz
// Provides: {"impl_200"}
// Dependencies: {}
impl StreamFooter { pub (crate) fn parse < R : Read > (reader : & mut R) -> crate :: Result < Self > { let expected_crc = reader . read_u32 () ? ; let backward_size = reader . read_u32 () ? ; let mut stream_flags = [0u8 ; 2] ; reader . read_exact (& mut stream_flags) ? ; let mut crc = CRC32 . digest () ; crc . update (& backward_size . to_le_bytes ()) ; crc . update (& stream_flags) ; if expected_crc != crc . finalize () { return Err (error_invalid_data ("stream footer CRC32 mismatch")) ; } let mut footer_magic = [0u8 ; 2] ; reader . read_exact (& mut footer_magic) ? ; if footer_magic != XZ_FOOTER_MAGIC { return Err (error_invalid_data ("invalid XZ footer magic bytes")) ; } Ok (StreamFooter { backward_size , stream_flags , }) } }
};
}
