// Generated macro for space (function)
macro_rules! Depcrate_parser_bytespace {
() => {
// Module: crate::parser::byte
// Provides: {"space"}
// Dependencies: {}
# [doc = " Parses a `b' '`, `b'\\t'`, `b'\\n'` or `'b\\'r'`."] # [doc = ""] # [doc = " ```"] # [doc = " use combine::Parser;"] # [doc = " use combine::parser::byte::space;"] # [doc = " assert_eq!(space().parse(&b\" \"[..]), Ok((b' ', &b\"\"[..])));"] # [doc = " assert_eq!(space().parse(&b\"  \"[..]), Ok((b' ', &b\" \"[..])));"] # [doc = " assert!(space().parse(&b\"!\"[..]).is_err());"] # [doc = " assert!(space().parse(&b\"\"[..]).is_err());"] # [doc = " ```"] pub fn space < Input > () -> impl Parser < Input , Output = u8 , PartialState = () > where Input : Stream < Token = u8 > , { byte_parser ! (space , Space , is_ascii_whitespace) }
};
}
