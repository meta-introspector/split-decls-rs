// Generated macro for subsection_escaped_char (function)
macro_rules! Depcrate_parse_nomsubsection_escaped_char {
() => {
// Module: crate::parse::nom
// Provides: {"subsection_escaped_char"}
// Dependencies: {}
fn subsection_escaped_char < 'i > (i : & mut & 'i [u8]) -> ModalResult < & 'i [u8] , NomError < & 'i [u8] > > { preceded ('\\' , one_of (is_subsection_escapable_char) . take ()) . parse_next (i) }
};
}
