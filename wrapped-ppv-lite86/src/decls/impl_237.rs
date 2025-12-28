macro_rules! impl_237 {
    () => {
        impl PartialEq for vec256_storage { # [inline (always)] fn eq (& self , rhs : & Self) -> bool { unsafe { self . sse2 == rhs . sse2 } } }
    };
}

impl_237!();