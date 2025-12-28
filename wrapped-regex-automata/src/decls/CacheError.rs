macro_rules! deps {
    () => {
        DFA!();
    };
}

macro_rules! CacheError {
    () => {
        deps!();
        # [doc = " An error that occurs when cache usage has become inefficient."] # [doc = ""] # [doc = " One of the weaknesses of a lazy DFA is that it may need to clear its"] # [doc = " cache repeatedly if it's not big enough. If this happens too much, then it"] # [doc = " can slow searching down significantly. A mitigation to this is to use"] # [doc = " heuristics to detect whether the cache is being used efficiently or not."] # [doc = " If not, then a lazy DFA can return a `CacheError`."] # [doc = ""] # [doc = " The default configuration of a lazy DFA in this crate is"] # [doc = " set such that a `CacheError` will never occur. Instead,"] # [doc = " callers must opt into this behavior with settings like"] # [doc = " [`dfa::Config::minimum_cache_clear_count`](crate::hybrid::dfa::Config::minimum_cache_clear_count)"] # [doc = " and"] # [doc = " [`dfa::Config::minimum_bytes_per_state`](crate::hybrid::dfa::Config::minimum_bytes_per_state)."] # [doc = ""] # [doc = " When the `std` feature is enabled, this implements the `std::error::Error`"] # [doc = " trait."] # [derive (Clone , Debug)] pub struct CacheError (()) ;
    };
}

CacheError!();