macro_rules! is_subsection_escapable_char {
    () => {
        fn is_subsection_escapable_char (c : u8) -> bool { c != b'\n' }
    };
}

is_subsection_escapable_char!();