// Generated macro for newline (function)
macro_rules! Depcrate_commit_message_decodenewline {
() => {
// Module: crate::commit::message::decode
// Provides: {"newline"}
// Dependencies: {}
pub (crate) fn newline < 'a , E : ParserError < & 'a [u8] > > (i : & mut & 'a [u8]) -> ModalResult < & 'a [u8] , E > { alt ((b"\n" , b"\r\n")) . parse_next (i) }
};
}
