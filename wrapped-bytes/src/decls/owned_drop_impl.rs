macro_rules! deps {
    () => {
        Owned!();
    };
}

macro_rules! owned_drop_impl {
    () => {
        deps!();
        unsafe fn owned_drop_impl < T > (owned : * mut ()) { { let ref_cnt = & * owned . cast :: < AtomicUsize > () ; let old_cnt = ref_cnt . fetch_sub (1 , Ordering :: Release) ; debug_assert ! (old_cnt > 0 && old_cnt <= usize :: MAX >> 1 , "expected non-zero refcount and no underflow") ; if old_cnt != 1 { return ; } ref_cnt . load (Ordering :: Acquire) ; } drop (Box :: < Owned < T > > :: from_raw (owned . cast ())) ; }
    };
}

owned_drop_impl!();