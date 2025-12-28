macro_rules! next_optional_percentage {
    () => {
        fn next_optional_percentage (i : & mut & [u8]) -> ModalResult < Option < u32 > , () > { opt (terminated (preceded (take_till (0 .. , | c : u8 | c . is_ascii_digit ()) , parse_number . try_map (u32 :: try_from) ,) , b"%" ,)) . parse_next (i) }
    };
}

next_optional_percentage!();