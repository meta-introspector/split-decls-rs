macro_rules! deps {
    () => {
        Shared!();
        Pointer!();
        Pointable!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl < T : ? Sized + Pointable > Pointer < T > for Shared < '_ , T > { # [inline] fn into_ptr (self) -> * mut () { self . data } # [inline] unsafe fn from_ptr (data : * mut ()) -> Self { Shared { data , _marker : PhantomData , } } }
    };
}

impl_53!()