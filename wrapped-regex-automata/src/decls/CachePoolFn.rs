macro_rules! deps {
    () => {
        Cache!();
    };
}

macro_rules! CachePoolFn {
    () => {
        deps!();
        # [doc = " The type of the closure we use to create new caches. We need to spell out"] # [doc = " all of the marker traits or else we risk leaking !MARKER impls."] type CachePoolFn = Box < dyn Fn () -> Cache + Send + Sync + UnwindSafe + RefUnwindSafe > ;
    };
}

CachePoolFn!();