macro_rules! first_wildcard_pos {
    () => {
        fn first_wildcard_pos (pat : & [u8]) -> Option < usize > { pat . find_byteset (GLOB_CHARACTERS) }
    };
}

first_wildcard_pos!();