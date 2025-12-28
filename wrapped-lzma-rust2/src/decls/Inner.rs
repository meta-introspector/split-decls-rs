macro_rules! Inner {
    () => {
        struct Inner < T > { queue : Mutex < VecDeque < T > > , condvar : Condvar , closed : AtomicBool , }
    };
}

Inner!()