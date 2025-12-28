macro_rules! subsection_subset {
    () => {
        fn subsection_subset < 'i > (i : & mut & 'i [u8]) -> ModalResult < & 'i [u8] , NomError < & 'i [u8] > > { alt ((subsection_unescaped , subsection_escaped_char)) . parse_next (i) }
    };
}

subsection_subset!()