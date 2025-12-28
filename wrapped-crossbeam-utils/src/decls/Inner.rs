macro_rules! deps {
    () => {
        WaitGroup!();
    };
}

macro_rules! Inner {
    () => {
        deps!();
        # [doc = " Inner state of a `WaitGroup`."] struct Inner { cvar : Condvar , count : Mutex < usize > , }
    };
}

Inner!()