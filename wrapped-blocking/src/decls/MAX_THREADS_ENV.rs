macro_rules! MAX_THREADS_ENV {
    () => {
        # [doc = " Env variable that allows to override default value for max threads."] # [cfg (not (target_family = "wasm"))] const MAX_THREADS_ENV : & str = "BLOCKING_MAX_THREADS" ;
    };
}

MAX_THREADS_ENV!()