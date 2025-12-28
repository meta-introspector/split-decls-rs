macro_rules! deps {
    () => {
        Whitespace!();
        ParseNode!();
        Event!();
    };
}

macro_rules! key_value_pair {
    () => {
        deps!();
        fn key_value_pair < 'i > (i : & mut & 'i [u8] , node : & mut ParseNode , dispatch : & mut dyn FnMut (Event < 'i >) ,) -> ModalResult < () , NomError < & 'i [u8] > > { * node = ParseNode :: Name ; if let Some (name) = opt (config_name) . parse_next (i) ? { dispatch (Event :: SectionValueName (section :: ValueName (Cow :: Borrowed (name)))) ; if let Some (whitespace) = opt (take_spaces1) . parse_next (i) ? { dispatch (Event :: Whitespace (Cow :: Borrowed (whitespace))) ; } * node = ParseNode :: Value ; config_value (i , dispatch) } else { Ok (()) } }
    };
}

key_value_pair!()