// Generated macro for write_byte_string (function)
macro_rules! Depcrate_utilswrite_byte_string {
() => {
// Module: crate::utils
// Provides: {"write_byte_string"}
// Dependencies: {}
pub fn write_byte_string (f : & mut Formatter , byte_string : & [u8]) -> fmt :: Result { write ! (f , "\"") ? ; for b in byte_string { match * b { 32 ..= 33 | 35 ..= 126 => write ! (f , "{}" , * b as char) ? , 34 => write ! (f , "\\\"") ? , _ => write ! (f , "{:#02X}" , b) ? , } } write ! (f , "\"") ? ; Ok (()) }
};
}
