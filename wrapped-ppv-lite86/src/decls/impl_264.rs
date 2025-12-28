macro_rules! impl_264 {
    () => {
        impl PartialEq < vec128_storage > for vec128_storage { # [inline (always)] fn eq (& self , rhs : & Self) -> bool { unsafe { self . q == rhs . q } } }
    };
}

impl_264!()