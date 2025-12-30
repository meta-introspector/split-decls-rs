// Generated macro for write_xz_stream_header (function)
macro_rules! Depcrate_xzwrite_xz_stream_header {
() => {
// Module: crate::xz
// Provides: {"write_xz_stream_header"}
// Dependencies: {}
# [cfg (feature = "encoder")] fn write_xz_stream_header < W : Write > (writer : & mut W , check_type : CheckType) -> crate :: Result < () > { writer . write_all (& XZ_MAGIC) ? ; let stream_flags = [0u8 , check_type as u8] ; writer . write_all (& stream_flags) ? ; let crc = CRC32 . checksum (& stream_flags) ; writer . write_u32 (crc) ? ; Ok (()) }
};
}
