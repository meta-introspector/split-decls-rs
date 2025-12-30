// Generated macro for gz_encoder (function)
macro_rules! Depcrate_gz_bufreadgz_encoder {
() => {
// Module: crate::gz::bufread
// Provides: {"gz_encoder"}
// Dependencies: {}
pub fn gz_encoder < R : BufRead > (header : Vec < u8 > , r : R , lvl : Compression) -> GzEncoder < R > { let crc = CrcReader :: new (r) ; GzEncoder { inner : deflate :: bufread :: DeflateEncoder :: new (crc , lvl) , header , pos : 0 , eof : false , } }
};
}
