macro_rules! backtrack {
    () => {
        # [cfg (feature = "nfa-backtrack")] pub mod backtrack ;
    };
}

backtrack!();