macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! shared_clone {
    () => {
        deps!();
        unsafe fn shared_clone (data : & AtomicPtr < () > , ptr : * const u8 , len : usize) -> Bytes { let shared = data . load (Ordering :: Relaxed) ; shallow_clone_arc (shared as _ , ptr , len) }
    };
}

shared_clone!();