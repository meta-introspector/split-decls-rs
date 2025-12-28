macro_rules! subsection_escaped_char {
    () => {
        fn subsection_escaped_char < 'i > (i : & mut & 'i [u8]) -> ModalResult < & 'i [u8] , NomError < & 'i [u8] > > { preceded ('\\' , one_of (is_subsection_escapable_char) . take ()) . parse_next (i) }
    };
}

subsection_escaped_char!();