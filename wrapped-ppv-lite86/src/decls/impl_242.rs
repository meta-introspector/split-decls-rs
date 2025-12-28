macro_rules! impl_242 {
    () => {
        impl PartialEq for vec512_storage { # [inline (always)] fn eq (& self , rhs : & Self) -> bool { unsafe { self . avx == rhs . avx } } }
    };
}

impl_242!()