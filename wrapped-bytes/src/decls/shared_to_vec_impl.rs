macro_rules! deps {
    () => {
        Shared!();
    };
}

macro_rules! shared_to_vec_impl {
    () => {
        deps!();
        unsafe fn shared_to_vec_impl (shared : * mut Shared , ptr : * const u8 , len : usize) -> Vec < u8 > { if (* shared) . ref_cnt . compare_exchange (1 , 0 , Ordering :: AcqRel , Ordering :: Relaxed) . is_ok () { let shared = * Box :: from_raw (shared) ; let shared = ManuallyDrop :: new (shared) ; let buf = shared . buf ; let cap = shared . cap ; ptr :: copy (ptr , buf , len) ; Vec :: from_raw_parts (buf , len , cap) } else { let v = slice :: from_raw_parts (ptr , len) . to_vec () ; release_shared (shared) ; v } }
    };
}

shared_to_vec_impl!();