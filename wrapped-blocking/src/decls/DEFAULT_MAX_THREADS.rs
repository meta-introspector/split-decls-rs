macro_rules! deps {
    () => {
        Executor!();
    };
}

macro_rules! DEFAULT_MAX_THREADS {
    () => {
        deps!();
        # [doc = " Default value for max threads that Executor can grow to"] # [cfg (not (target_family = "wasm"))] const DEFAULT_MAX_THREADS : NonZeroUsize = { if let Some (size) = NonZeroUsize :: new (500) { size } else { panic ! ("DEFAULT_MAX_THREADS is non-zero") ; } } ;
    };
}

DEFAULT_MAX_THREADS!();