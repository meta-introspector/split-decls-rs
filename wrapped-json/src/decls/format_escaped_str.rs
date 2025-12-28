macro_rules! deps {
    () => {
        Result!();
        Formatter!();
    };
}

macro_rules! format_escaped_str {
    () => {
        deps!();
        fn format_escaped_str < W , F > (writer : & mut W , formatter : & mut F , value : & str) -> io :: Result < () > where W : ? Sized + io :: Write , F : ? Sized + Formatter , { tri ! (formatter . begin_string (writer)) ; tri ! (format_escaped_str_contents (writer , formatter , value)) ; formatter . end_string (writer) }
    };
}

format_escaped_str!()