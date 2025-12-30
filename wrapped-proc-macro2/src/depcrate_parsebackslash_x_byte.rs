// Generated macro for backslash_x_byte (function)
macro_rules! Depcrate_parsebackslash_x_byte {
() => {
// Module: crate::parse
// Provides: {"backslash_x_byte"}
// Dependencies: {}
fn backslash_x_byte < I > (chars : & mut I) -> Result < () , Reject > where I : Iterator < Item = (usize , u8) > , { next_ch ! (chars @ b'0' ..= b'9' | b'a' ..= b'f' | b'A' ..= b'F') ; next_ch ! (chars @ b'0' ..= b'9' | b'a' ..= b'f' | b'A' ..= b'F') ; Ok (()) }
};
}
