// Generated macro for len (function)
macro_rules! Depcrate_util_utf8len {
() => {
// Module: crate::util::utf8
// Provides: {"len"}
// Dependencies: {}
# [doc = " Given a UTF-8 leading byte, this returns the total number of code units"] # [doc = " in the following encoded codepoint."] # [doc = ""] # [doc = " If the given byte is not a valid UTF-8 leading byte, then this returns"] # [doc = " `None`."] # [cfg_attr (feature = "perf-inline" , inline (always))] fn len (byte : u8) -> Option < usize > { match byte { 0b0000_0000 ..= 0b0111_1111 => Some (1) , 0b1000_0000 ..= 0b1011_1111 => None , 0b1100_0000 ..= 0b1101_1111 => Some (2) , 0b1110_0000 ..= 0b1110_1111 => Some (3) , 0b1111_0000 ..= 0b1111_0111 => Some (4) , _ => None , } }
};
}
