// Generated macro for config_value (function)
macro_rules! Depcrate_parse_nomconfig_value {
() => {
// Module: crate::parse::nom
// Provides: {"config_value"}
// Dependencies: {}
fn config_value < 'i > (i : & mut & 'i [u8] , dispatch : & mut dyn FnMut (Event < 'i >)) -> ModalResult < () , NomError < & 'i [u8] > > { if opt ('=') . parse_next (i) ? . is_some () { dispatch (Event :: KeyValueSeparator) ; if let Some (whitespace) = opt (take_spaces1) . parse_next (i) ? { dispatch (Event :: Whitespace (Cow :: Borrowed (whitespace))) ; } value_impl (i , dispatch) } else { dispatch (Event :: Value (Cow :: Borrowed ("" . into ()))) ; Ok (()) } }
};
}
