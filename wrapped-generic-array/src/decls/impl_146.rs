macro_rules! deps {
    () => {
        ArrayBuilder!();
        ArrayLength!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        impl < T , N : ArrayLength > Drop for ArrayBuilder < T , N > { fn drop (& mut self) { unsafe { ptr :: drop_in_place (self . array . get_unchecked_mut (.. self . position) as * mut [MaybeUninit < T >] as * mut [T] ,) ; } } }
    };
}

impl_146!();