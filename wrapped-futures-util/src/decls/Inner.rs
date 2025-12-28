macro_rules! Inner {
    () => {
        # [derive (Debug)] struct Inner < T > { state : AtomicPtr < Waker > , value : Option < UnsafeCell < T > > , }
    };
}

Inner!();