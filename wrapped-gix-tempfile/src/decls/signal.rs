macro_rules! signal {
    () => {
        # [doc = " signal setup and reusable handlers."] # [cfg (feature = "signals")] pub mod signal ;
    };
}

signal!()