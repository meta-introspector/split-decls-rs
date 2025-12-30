// Generated macro for byte (function)
macro_rules! Depcrate_parser_bytebyte {
() => {
// Module: crate::parser::byte
// Provides: {"byte"}
// Dependencies: {}
# [doc = " Parses a byte and succeeds if the byte is equal to `c`."] # [doc = ""] # [doc = " ```"] # [doc = " use combine::Parser;"] # [doc = " use combine::parser::byte::byte;"] # [doc = " assert_eq!(byte(b'!').parse(&b\"!\"[..]), Ok((b'!', &b\"\"[..])));"] # [doc = " assert!(byte(b'A').parse(&b\"\"[..]).is_err());"] # [doc = " assert!(byte(b'A').parse(&b\"!\"[..]).is_err());"] # [doc = " ```"] pub fn byte < Input > (c : u8) -> Token < Input > where Input : Stream < Token = u8 > , { token (c) }
};
}
