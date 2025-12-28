macro_rules! deps {
    () => {
        Shared!();
    };
}

macro_rules! shared_v_is_unique {
    () => {
        deps!();
        unsafe fn shared_v_is_unique (data : & AtomicPtr < () >) -> bool { let shared = data . load (Ordering :: Acquire) ; let ref_count = (* shared . cast :: < Shared > ()) . ref_count . load (Ordering :: Relaxed) ; ref_count == 1 }
    };
}

shared_v_is_unique!();