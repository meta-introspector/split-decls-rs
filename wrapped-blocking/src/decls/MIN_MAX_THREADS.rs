macro_rules! MIN_MAX_THREADS {
    () => {
        # [doc = " Minimum value for max threads config"] # [cfg (not (target_family = "wasm"))] const MIN_MAX_THREADS : usize = 1 ;
    };
}

MIN_MAX_THREADS!();