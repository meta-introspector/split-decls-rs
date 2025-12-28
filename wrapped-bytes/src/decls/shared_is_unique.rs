macro_rules! deps {
    () => {
        Shared!();
    };
}

macro_rules! shared_is_unique {
    () => {
        deps!();
        pub (crate) unsafe fn shared_is_unique (data : & AtomicPtr < () >) -> bool { let shared = data . load (Ordering :: Acquire) ; let ref_cnt = (* shared . cast :: < Shared > ()) . ref_cnt . load (Ordering :: Relaxed) ; ref_cnt == 1 }
    };
}

shared_is_unique!()