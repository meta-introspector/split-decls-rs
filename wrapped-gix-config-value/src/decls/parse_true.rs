macro_rules! parse_true {
    () => {
        fn parse_true (value : & BStr) -> bool { value . eq_ignore_ascii_case (b"yes") || value . eq_ignore_ascii_case (b"on") || value . eq_ignore_ascii_case (b"true") }
    };
}

parse_true!()