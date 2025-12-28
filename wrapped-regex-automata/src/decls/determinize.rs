macro_rules! determinize {
    () => {
        # [cfg (any (feature = "dfa-build" , feature = "hybrid"))] pub (crate) mod determinize ;
    };
}

determinize!()