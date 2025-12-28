macro_rules! disabled {
    () => {
        # [cfg (not (feature = "tracing"))] mod disabled ;
    };
}

disabled!()