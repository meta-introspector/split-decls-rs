// Generated macro for section (function)
macro_rules! Depcrate_parse_nomsection {
() => {
// Module: crate::parse::nom
// Provides: {"section"}
// Dependencies: {}
fn section < 'i > (i : & mut & 'i [u8] , node : & mut ParseNode , dispatch : & mut dyn FnMut (Event < 'i >) ,) -> ModalResult < () , NomError < & 'i [u8] > > { let start = i . checkpoint () ; let header = section_header (i) . inspect_err (| _err | { i . reset (& start) ; }) ? ; dispatch (Event :: SectionHeader (header)) ; loop { let start = i . checkpoint () ; if let Some (v) = opt (take_spaces1) . parse_next (i) ? { dispatch (Event :: Whitespace (Cow :: Borrowed (v . as_bstr ()))) ; } if let Some (v) = opt (take_newlines1) . parse_next (i) ? { dispatch (Event :: Newline (Cow :: Borrowed (v . as_bstr ()))) ; } key_value_pair (i , node , dispatch) ? ; if let Some (comment) = opt (comment) . parse_next (i) ? { dispatch (Event :: Comment (comment)) ; } if i . offset_from (& start) == 0 { break ; } } Ok (()) }
};
}
