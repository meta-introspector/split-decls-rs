macro_rules! deps {
    () => {
        Pointer!();
    };
}

macro_rules! impl_285 {
    () => {
        deps!();
        impl < T > Pointer for * const T { unsafe fn distance (self , origin : * const T) -> usize { usize :: try_from (self . offset_from (origin)) . unwrap_unchecked () } fn as_usize (self) -> usize { self as usize } }
    };
}

impl_285!();