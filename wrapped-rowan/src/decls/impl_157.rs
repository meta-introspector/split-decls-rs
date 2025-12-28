macro_rules! deps {
    () => {
        ThinArc!();
        HeaderSlice!();
        ArcInner!();
        Arc!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl < H , T > Arc < HeaderSlice < H , [T] > > { # [doc = " Converts an `Arc` into a `ThinArc`. This consumes the `Arc`, so the refcount"] # [doc = " is not modified."] # [inline] pub (crate) fn into_thin (a : Self) -> ThinArc < H , T > { assert_eq ! (a . length , a . slice . len () , "Length needs to be correct for ThinArc to work") ; let fat_ptr : * mut ArcInner < HeaderSlice < H , [T] > > = a . ptr () ; mem :: forget (a) ; let thin_ptr = fat_ptr as * mut [usize] as * mut usize ; ThinArc { ptr : unsafe { ptr :: NonNull :: new_unchecked (thin_ptr as * mut ArcInner < HeaderSlice < H , [T ; 0] > >) } , phantom : PhantomData , } } # [doc = " Converts a `ThinArc` into an `Arc`. This consumes the `ThinArc`, so the refcount"] # [doc = " is not modified."] # [inline] pub (crate) fn from_thin (a : ThinArc < H , T >) -> Self { let ptr = thin_to_thick (a . ptr . as_ptr ()) ; mem :: forget (a) ; unsafe { Arc { p : ptr :: NonNull :: new_unchecked (ptr) , phantom : PhantomData } } } }
    };
}

impl_157!();