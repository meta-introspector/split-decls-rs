// Generated macro for impl_385 (impl)
macro_rules! Depcrate_parser_elementimpl_385 {
() => {
// Module: crate::parser::element
// Provides: {"impl_385"}
// Dependencies: {}
impl Parser for ElementParser { # [doc = " Returns number of consumed bytes or `None` if `>` was not found in `bytes`."] # [inline] fn feed (& mut self , bytes : & [u8]) -> Option < usize > { for i in memchr :: memchr3_iter (b'>' , b'\'' , b'"' , bytes) { * self = match (* self , bytes [i]) { (Self :: Outside , b'>') => return Some (i) , (Self :: Outside , b'\'') => Self :: SingleQ , (Self :: Outside , b'\"') => Self :: DoubleQ , (Self :: SingleQ , b'\'') | (Self :: DoubleQ , b'"') => Self :: Outside , _ => continue , } ; } None } # [inline] fn eof_error () -> SyntaxError { SyntaxError :: UnclosedTag } }
};
}
