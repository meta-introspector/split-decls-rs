macro_rules! deps {
    () => {
        SmallCString!();
    };
}

macro_rules! impl_666 {
    () => {
        deps!();
        impl std :: ops :: Deref for SmallCString { type Target = CStr ; # [inline] fn deref (& self) -> & CStr { self . as_cstr () } }
    };
}

impl_666!();