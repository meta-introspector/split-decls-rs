// Generated macro for header_field (function)
macro_rules! Depcrate_parseheader_field {
() => {
// Module: crate::parse
// Provides: {"header_field"}
// Dependencies: {}
pub (crate) fn header_field < 'a , T , E : ParserError < & 'a [u8] > > (i : & mut & 'a [u8] , name : & 'static [u8] , parse_value : impl ModalParser < & 'a [u8] , T , E > ,) -> ModalResult < T , E > { terminated (preceded (terminated (name , SPACE) , parse_value) , NL) . parse_next (i) }
};
}
