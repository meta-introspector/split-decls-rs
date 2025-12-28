macro_rules! deps {
    () => {
        Data!();
    };
}

macro_rules! Deferred {
    () => {
        deps!();
        # [doc = " A `FnOnce()` that is stored inline if small, or otherwise boxed on the heap."] # [doc = ""] # [doc = " This is a handy way of keeping an unsized `FnOnce()` within a sized structure."] pub (crate) struct Deferred { call : unsafe fn (* mut u8) , data : MaybeUninit < Data > , _marker : PhantomData < * mut () > , }
    };
}

Deferred!();