macro_rules! parse_false {
    () => {
        fn parse_false (value : & BStr) -> bool { value . eq_ignore_ascii_case (b"no") || value . eq_ignore_ascii_case (b"off") || value . eq_ignore_ascii_case (b"false") || value . is_empty () }
    };
}

parse_false!()