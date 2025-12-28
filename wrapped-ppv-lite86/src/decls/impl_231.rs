macro_rules! impl_231 {
    () => {
        impl PartialEq for vec128_storage { # [inline (always)] fn eq (& self , rhs : & Self) -> bool { unsafe { self . u128x1 == rhs . u128x1 } } }
    };
}

impl_231!()