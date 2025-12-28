macro_rules! deps {
    () => {
        Arc!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl < T : ? Sized > Clone for Arc < T > { # [inline] fn clone (& self) -> Self { let old_size = self . inner () . count . fetch_add (1 , Relaxed) ; if old_size > MAX_REFCOUNT { std :: process :: abort () ; } unsafe { Arc { p : ptr :: NonNull :: new_unchecked (self . ptr ()) , phantom : PhantomData } } } }
    };
}

impl_137!();