// Generated macro for Check (enum)
macro_rules! Depcrate_streamCheck {
() => {
// Module: crate::stream
// Provides: {"Check"}
// Dependencies: {}
# [doc = " Possible integrity checks that can be part of a .xz stream."] # [allow (missing_docs)] # [derive (Debug , Copy , Clone)] pub enum Check { None = liblzma_sys :: LZMA_CHECK_NONE as isize , Crc32 = liblzma_sys :: LZMA_CHECK_CRC32 as isize , Crc64 = liblzma_sys :: LZMA_CHECK_CRC64 as isize , Sha256 = liblzma_sys :: LZMA_CHECK_SHA256 as isize , }
};
}
