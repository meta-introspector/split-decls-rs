macro_rules! deps {
    () => {
        IntrusiveArrayBuilder!();
        ArrayLength!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl < T , N : ArrayLength > Drop for IntrusiveArrayBuilder < '_ , T , N > { fn drop (& mut self) { unsafe { ptr :: drop_in_place (self . array . get_unchecked_mut (.. self . position) as * mut [MaybeUninit < T >] as * mut [T] ,) ; } } }
    };
}

impl_149!();