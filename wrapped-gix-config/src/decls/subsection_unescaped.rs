macro_rules! subsection_unescaped {
    () => {
        fn subsection_unescaped < 'i > (i : & mut & 'i [u8]) -> ModalResult < & 'i [u8] , NomError < & 'i [u8] > > { take_while (1 .. , is_subsection_unescaped_char) . parse_next (i) }
    };
}

subsection_unescaped!();