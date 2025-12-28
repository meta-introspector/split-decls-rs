macro_rules! is_subsection_unescaped_char {
    () => {
        fn is_subsection_unescaped_char (c : u8) -> bool { c != b'"' && c != b'\\' && c != b'\n' && c != 0 }
    };
}

is_subsection_unescaped_char!();