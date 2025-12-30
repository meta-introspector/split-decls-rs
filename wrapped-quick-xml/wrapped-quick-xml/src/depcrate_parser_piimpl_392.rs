// Generated macro for impl_392 (impl)
macro_rules! Depcrate_parser_piimpl_392 {
() => {
// Module: crate::parser::pi
// Provides: {"impl_392"}
// Dependencies: {}
impl Parser for PiParser { # [doc = " Determines the end position of a processing instruction in the provided slice."] # [doc = " Processing instruction ends on the first occurrence of `?>` which cannot be"] # [doc = " escaped."] # [doc = ""] # [doc = " Returns position after the `?>` or `None` if such sequence was not found."] # [doc = ""] # [doc = " [Section 2.6]: Parameter entity references MUST NOT be recognized within"] # [doc = " processing instructions, so parser do not search for them."] # [doc = ""] # [doc = " # Parameters"] # [doc = " - `bytes`: a slice to find the end of a processing instruction."] # [doc = "   Should contain text in ASCII-compatible encoding"] # [doc = ""] # [doc = " [Section 2.6]: https://www.w3.org/TR/xml11/#sec-pi"] # [inline] fn feed (& mut self , bytes : & [u8]) -> Option < usize > { for i in memchr :: memchr_iter (b'>' , bytes) { match i { 0 if self . 0 => return Some (0) , i if i > 0 && bytes [i - 1] == b'?' => return Some (i) , _ => { } } } self . 0 = bytes . last () . copied () == Some (b'?') ; None } # [inline] fn eof_error () -> SyntaxError { SyntaxError :: UnclosedPIOrXmlDecl } }
};
}
