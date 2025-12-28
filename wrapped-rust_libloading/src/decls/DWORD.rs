macro_rules! DWORD {
    () => {
        # [allow (clippy :: upper_case_acronyms)] type DWORD = u32 ;
    };
}

DWORD!();