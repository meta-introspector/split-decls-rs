macro_rules! dfa {
    () => {
        # [cfg (any (feature = "dfa-search" , feature = "dfa-onepass"))] pub mod dfa ;
    };
}

dfa!()