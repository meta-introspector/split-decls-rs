// Generated macro for len (function)
macro_rules! Depcrate_util_utf8len {
() => {
// Module: crate::util::utf8
// Provides: {"len"}
// Dependencies: {}
# [doc = " Given a UTF-8 leading byte, this returns the total number of code units"] # [doc = " in the following encoded codepoint."] # [doc = ""] # [doc = " If the given byte is not a valid UTF-8 leading byte, then this returns"] # [doc = " `None`."] # [cfg_attr (feature = "perf-inline" , inline (always))] fn len (byte : u8) -> Option < usize > { if byte <= 0x7F { return Some (1) ; } else if byte & 0b1100_0000 == 0b1000_0000 { return None ; } else if byte <= 0b1101_1111 { Some (2) } else if byte <= 0b1110_1111 { Some (3) } else if byte <= 0b1111_0111 { Some (4) } else { None } }
};
}
