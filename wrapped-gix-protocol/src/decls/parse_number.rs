macro_rules! parse_number {
    () => {
        fn parse_number (i : & mut & [u8]) -> ModalResult < usize , () > { take_till (0 .. , | c : u8 | ! c . is_ascii_digit ()) . try_map (gix_utils :: btoi :: to_signed) . parse_next (i) }
    };
}

parse_number!()