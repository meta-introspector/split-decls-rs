macro_rules! atomic_load {
    () => {
        # [doc = " Atomically reads data from `src`."] # [doc = ""] # [doc = " This operation uses the `Acquire` ordering. If possible, an atomic instructions is used, and a"] # [doc = " global lock otherwise."] unsafe fn atomic_load < T > (src : * mut T) -> T where T : Copy , { atomic ! { T , a , { a = unsafe { &* (src as * const _ as * const _) } ; unsafe { mem :: transmute_copy (& a . load (Ordering :: Acquire)) } } , { let lock = lock (src as usize) ; if let Some (stamp) = lock . optimistic_read () { let val = unsafe { ptr :: read_volatile (src . cast ::< MaybeUninit < T >> ()) } ; if lock . validate_read (stamp) { return unsafe { val . assume_init () } ; } } let guard = lock . write () ; let val = unsafe { ptr :: read (src) } ; guard . abort () ; val } } }
    };
}

atomic_load!()