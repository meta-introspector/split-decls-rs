macro_rules! deps {
    () => {
        Shared!();
    };
}

macro_rules! release_shared {
    () => {
        deps!();
        unsafe fn release_shared (ptr : * mut Shared) { if (* ptr) . ref_count . fetch_sub (1 , Ordering :: Release) != 1 { return ; } (* ptr) . ref_count . load (Ordering :: Acquire) ; drop (Box :: from_raw (ptr)) ; }
    };
}

release_shared!();