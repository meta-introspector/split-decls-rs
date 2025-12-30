// Generated macro for newline (function)
macro_rules! Depcrate_parsenewline {
() => {
// Module: crate::parse
// Provides: {"newline"}
// Dependencies: {}
pub fn newline < 'a , E : ParserError < & 'a [u8] > > (i : & mut & 'a [u8]) -> ModalResult < & 'a [u8] , E > { alt ((b"\r\n" , b"\n")) . parse_next (i) }
};
}
