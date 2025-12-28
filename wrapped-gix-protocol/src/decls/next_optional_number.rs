macro_rules! next_optional_number {
    () => {
        fn next_optional_number (i : & mut & [u8]) -> ModalResult < Option < usize > , () > { opt (preceded (take_till (0 .. , | c : u8 | c . is_ascii_digit ()) , parse_number)) . parse_next (i) }
    };
}

next_optional_number!();