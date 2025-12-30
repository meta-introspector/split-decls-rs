// Generated macro for reset_decoder_data (function)
macro_rules! Depcrate_zlib_bufreadreset_decoder_data {
() => {
// Module: crate::zlib::bufread
// Provides: {"reset_decoder_data"}
// Dependencies: {}
pub fn reset_decoder_data < R > (zlib : & mut ZlibDecoder < R >) { zlib . data = Decompress :: new (true) ; }
};
}
