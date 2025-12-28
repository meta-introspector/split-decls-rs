macro_rules! HMODULE {
    () => {
        # [allow (clippy :: upper_case_acronyms)] type HMODULE = isize ;
    };
}

HMODULE!();