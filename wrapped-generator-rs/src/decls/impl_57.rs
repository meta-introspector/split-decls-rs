macro_rules! deps {
    () => {
        Stack!();
        Func!();
        StackBox!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < F : FnOnce () > StackBox < F > { fn call_once (data : * mut ()) { unsafe { let data = data as * mut F ; let f = data . read () ; f () ; } } fn drop_inner (data : * mut ()) { unsafe { let data = data as * mut F ; ptr :: drop_in_place (data) ; } } # [doc = " create a functor on the stack"] pub (crate) fn new_fn_once (stack : & mut Stack , data : F) -> Func { unsafe { let mut d = Self :: new_uninit (stack , 0) ; (* d . as_mut_ptr ()) . init (data) ; let d = d . assume_init () ; let header = d . get_header () ; let f = Func { data : d . ptr . as_ptr () as * mut () , size : header . data_size + HEADER_SIZE , offset : stack . get_offset () , func : Self :: call_once , drop : Self :: drop_inner , } ; std :: mem :: forget (d) ; f } } }
    };
}

impl_57!()