// Generated macro for subsection_unescaped (function)
macro_rules! Depcrate_parse_nomsubsection_unescaped {
() => {
// Module: crate::parse::nom
// Provides: {"subsection_unescaped"}
// Dependencies: {}
fn subsection_unescaped < 'i > (i : & mut & 'i [u8]) -> ModalResult < & 'i [u8] , NomError < & 'i [u8] > > { take_while (1 .. , is_subsection_unescaped_char) . parse_next (i) }
};
}
