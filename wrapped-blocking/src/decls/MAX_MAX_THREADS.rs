macro_rules! MAX_MAX_THREADS {
    () => {
        # [doc = " Maximum value for max threads config"] # [cfg (not (target_family = "wasm"))] const MAX_MAX_THREADS : usize = 10000 ;
    };
}

MAX_MAX_THREADS!()