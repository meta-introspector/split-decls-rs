macro_rules! minimize {
    () => {
        # [cfg (feature = "dfa-build")] mod minimize ;
    };
}

minimize!()