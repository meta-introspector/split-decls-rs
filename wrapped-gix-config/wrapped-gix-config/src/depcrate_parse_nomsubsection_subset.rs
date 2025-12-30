// Generated macro for subsection_subset (function)
macro_rules! Depcrate_parse_nomsubsection_subset {
() => {
// Module: crate::parse::nom
// Provides: {"subsection_subset"}
// Dependencies: {}
fn subsection_subset < 'i > (i : & mut & 'i [u8]) -> ModalResult < & 'i [u8] , NomError < & 'i [u8] > > { alt ((subsection_unescaped , subsection_escaped_char)) . parse_next (i) }
};
}
