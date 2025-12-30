// Generated macro for decode_inner (function)
macro_rules! Depcratedecode_inner {
() => {
// Module: crate
// Provides: {"decode_inner"}
// Dependencies: {}
fn decode_inner < 'a > (src : & [u8] , dst : & 'a mut [u8] , decode_nibble : impl Fn (u8) -> u16 ,) -> Result < & 'a [u8] > { let dst = dst . get_mut (.. decoded_len (src) ?) . ok_or (Error :: InvalidLength) ? ; let mut err : u16 = 0 ; for (src , dst) in src . chunks_exact (2) . zip (dst . iter_mut ()) { let byte = (decode_nibble (src [0]) << 4) | decode_nibble (src [1]) ; err |= byte >> 8 ; * dst = byte as u8 ; } match err { 0 => Ok (dst) , _ => Err (Error :: InvalidEncoding) , } }
};
}
