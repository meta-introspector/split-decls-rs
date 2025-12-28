macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! promotable_odd_clone {
    () => {
        deps!();
        unsafe fn promotable_odd_clone (data : & AtomicPtr < () > , ptr : * const u8 , len : usize) -> Bytes { let shared = data . load (Ordering :: Acquire) ; let kind = shared as usize & KIND_MASK ; if kind == KIND_ARC { shallow_clone_arc (shared as _ , ptr , len) } else { debug_assert_eq ! (kind , KIND_VEC) ; shallow_clone_vec (data , shared , shared . cast () , ptr , len) } }
    };
}

promotable_odd_clone!();