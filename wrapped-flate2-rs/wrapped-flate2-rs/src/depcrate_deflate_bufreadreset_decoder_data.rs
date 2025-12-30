// Generated macro for reset_decoder_data (function)
macro_rules! Depcrate_deflate_bufreadreset_decoder_data {
() => {
// Module: crate::deflate::bufread
// Provides: {"reset_decoder_data"}
// Dependencies: {}
pub fn reset_decoder_data < R > (zlib : & mut DeflateDecoder < R >) { zlib . data = Decompress :: new (false) ; }
};
}
