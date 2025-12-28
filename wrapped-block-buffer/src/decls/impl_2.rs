macro_rules! deps {
    () => {
        ReadBuffer!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl < BS : ArraySize > Default for ReadBuffer < BS > { # [inline] fn default () -> Self { assert ! (BS :: USIZE != 0 && BS :: USIZE < 256 , "buffer block size must be bigger than zero and smaller than 256") ; let buffer = Default :: default () ; let mut res = Self { buffer } ; unsafe { res . set_pos_unchecked (BS :: USIZE) } ; res } }
    };
}

impl_2!();