macro_rules! deps {
    () => {
        CachedDate!();
    };
}

macro_rules! macro_52 {
    () => {
        deps!();
        thread_local ! (static CACHED : RefCell < CachedDate > = RefCell :: new (CachedDate :: new ())) ;
    };
}

macro_52!();