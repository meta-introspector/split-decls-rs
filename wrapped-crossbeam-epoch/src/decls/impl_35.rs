macro_rules! deps {
    () => {
        Owned!();
        Pointer!();
        Pointable!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < T : ? Sized + Pointable > Pointer < T > for Owned < T > { # [inline] fn into_ptr (self) -> * mut () { let data = self . data ; mem :: forget (self) ; data } # [doc = " Returns a new pointer pointing to the tagged pointer `data`."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the pointer is null, but only in debug mode."] # [inline] unsafe fn from_ptr (data : * mut ()) -> Self { debug_assert ! (! data . is_null () , "converting null into `Owned`") ; Self { data , _marker : PhantomData , } } }
    };
}

impl_35!()