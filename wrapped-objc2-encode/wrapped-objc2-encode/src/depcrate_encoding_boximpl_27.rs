// Generated macro for impl_27 (impl)
macro_rules! Depcrate_encoding_boximpl_27 {
() => {
// Module: crate::encoding_box
// Provides: {"impl_27"}
// Dependencies: {}
impl FromStr for EncodingBox { type Err = ParseError ; fn from_str (s : & str) -> Result < Self , Self :: Err > { let mut parser = Parser :: new (s) ; parser . strip_leading_qualifiers () ; parser . parse_encoding_or_none () . and_then (| enc | parser . expect_empty () . map (| () | enc)) . map_err (| err | ParseError :: new (parser , err)) } }
};
}
