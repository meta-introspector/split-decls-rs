macro_rules! atomic_store {
    () => {
        # [doc = " Atomically writes `val` to `dst`."] # [doc = ""] # [doc = " This operation uses the `Release` ordering. If possible, an atomic instructions is used, and a"] # [doc = " global lock otherwise."] unsafe fn atomic_store < T > (dst : * mut T , val : T) { atomic ! { T , a , { a = unsafe { &* (dst as * const _ as * const _) } ; a . store (unsafe { mem :: transmute_copy (& val) } , Ordering :: Release) ; mem :: forget (val) ; } , { let _guard = lock (dst as usize) . write () ; unsafe { ptr :: write (dst , val) } } } }
    };
}

atomic_store!()