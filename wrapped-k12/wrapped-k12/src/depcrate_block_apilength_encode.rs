// Generated macro for length_encode (function)
macro_rules! Depcrate_block_apilength_encode {
() => {
// Module: crate::block_api
// Provides: {"length_encode"}
// Dependencies: {}
fn length_encode (mut length : usize , buffer : & mut [u8 ; LENGTH_ENCODE_SIZE]) -> & mut [u8] { let mut bufpos = 0usize ; while length > 0 { buffer [bufpos] = (length % 256) as u8 ; length /= 256 ; bufpos += 1 ; } buffer [.. bufpos] . reverse () ; buffer [bufpos] = bufpos as u8 ; bufpos += 1 ; & mut buffer [.. bufpos] }
};
}
