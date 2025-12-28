macro_rules! Backtrace {
    () => {
        # [cfg (not (feature = "debug"))] # [derive (Debug)] struct Backtrace ;
    };
}

Backtrace!()