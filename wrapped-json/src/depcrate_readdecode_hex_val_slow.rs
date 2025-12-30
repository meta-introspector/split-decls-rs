// Generated macro for decode_hex_val_slow (function)
macro_rules! Depcrate_readdecode_hex_val_slow {
() => {
// Module: crate::read
// Provides: {"decode_hex_val_slow"}
// Dependencies: {}
const fn decode_hex_val_slow (val : u8) -> Option < u8 > { match val { b'0' ..= b'9' => Some (val - b'0') , b'A' ..= b'F' => Some (val - b'A' + 10) , b'a' ..= b'f' => Some (val - b'a' + 10) , _ => None , } }
};
}
