macro_rules! stopat {
    () => {
        # [cfg (any (feature = "dfa-build" , feature = "hybrid"))] mod stopat ;
    };
}

stopat!()