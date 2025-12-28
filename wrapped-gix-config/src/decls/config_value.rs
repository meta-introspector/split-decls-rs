macro_rules! deps {
    () => {
        Whitespace!();
        Event!();
    };
}

macro_rules! config_value {
    () => {
        deps!();
        fn config_value < 'i > (i : & mut & 'i [u8] , dispatch : & mut dyn FnMut (Event < 'i >)) -> ModalResult < () , NomError < & 'i [u8] > > { if opt ('=') . parse_next (i) ? . is_some () { dispatch (Event :: KeyValueSeparator) ; if let Some (whitespace) = opt (take_spaces1) . parse_next (i) ? { dispatch (Event :: Whitespace (Cow :: Borrowed (whitespace))) ; } value_impl (i , dispatch) } else { dispatch (Event :: Value (Cow :: Borrowed ("" . into ()))) ; Ok (()) } }
    };
}

config_value!()