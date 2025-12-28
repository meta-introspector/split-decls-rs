macro_rules! deps {
    () => {
        Cache!();
        DFA!();
    };
}

macro_rules! LazyRef {
    () => {
        deps!();
        # [doc = " A type that groups methods that require the base NFA/DFA and read-only"] # [doc = " access to the cache."] # [derive (Debug)] struct LazyRef < 'i , 'c > { dfa : & 'i DFA , cache : & 'c Cache , }
    };
}

LazyRef!();