macro_rules! FARPROC {
    () => {
        # [allow (clippy :: upper_case_acronyms)] type FARPROC = Option < unsafe extern "system" fn () -> isize > ;
    };
}

FARPROC!()