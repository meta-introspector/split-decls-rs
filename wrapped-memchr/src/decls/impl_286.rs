macro_rules! deps {
    () => {
        Pointer!();
    };
}

macro_rules! impl_286 {
    () => {
        deps!();
        impl < T > Pointer for * mut T { unsafe fn distance (self , origin : * mut T) -> usize { (self as * const T) . distance (origin as * const T) } fn as_usize (self) -> usize { (self as * const T) . as_usize () } }
    };
}

impl_286!()