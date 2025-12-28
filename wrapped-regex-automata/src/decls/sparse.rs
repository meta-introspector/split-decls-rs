macro_rules! sparse {
    () => {
        # [cfg (feature = "dfa-search")] pub mod sparse ;
    };
}

sparse!();