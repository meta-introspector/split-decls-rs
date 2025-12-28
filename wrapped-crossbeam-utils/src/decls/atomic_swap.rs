macro_rules! atomic_swap {
    () => {
        # [doc = " Atomically swaps data at `dst` with `val`."] # [doc = ""] # [doc = " This operation uses the `AcqRel` ordering. If possible, an atomic instructions is used, and a"] # [doc = " global lock otherwise."] unsafe fn atomic_swap < T > (dst : * mut T , val : T) -> T { atomic ! { T , a , { a = unsafe { &* (dst as * const _ as * const _) } ; let res = unsafe { mem :: transmute_copy (& a . swap (mem :: transmute_copy (& val) , Ordering :: AcqRel)) } ; mem :: forget (val) ; res } , { let _guard = lock (dst as usize) . write () ; unsafe { ptr :: replace (dst , val) } } } }
    };
}

atomic_swap!();