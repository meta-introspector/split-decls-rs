macro_rules! deps {
    () => {
        Shared!();
    };
}

macro_rules! shared_v_to_vec {
    () => {
        deps!();
        unsafe fn shared_v_to_vec (data : & AtomicPtr < () > , ptr : * const u8 , len : usize) -> Vec < u8 > { let shared : * mut Shared = data . load (Ordering :: Relaxed) . cast () ; if (* shared) . is_unique () { let shared = & mut * shared ; let mut vec = core :: mem :: take (& mut shared . vec) ; release_shared (shared) ; ptr :: copy (ptr , vec . as_mut_ptr () , len) ; vec . set_len (len) ; vec } else { let v = slice :: from_raw_parts (ptr , len) . to_vec () ; release_shared (shared) ; v } }
    };
}

shared_v_to_vec!();