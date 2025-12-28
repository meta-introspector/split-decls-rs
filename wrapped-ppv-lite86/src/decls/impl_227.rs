macro_rules! impl_227 {
    () => {
        impl < 'a > From < & 'a vec128_storage > for & 'a [u32 ; 4] { # [inline (always)] fn from (x : & 'a vec128_storage) -> Self { unsafe { & x . u32x4 } } }
    };
}

impl_227!()