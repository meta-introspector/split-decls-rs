macro_rules! limited {
    () => {
        # [cfg (any (feature = "dfa-build" , feature = "hybrid"))] mod limited ;
    };
}

limited!();