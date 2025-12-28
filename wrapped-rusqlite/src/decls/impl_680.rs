macro_rules! deps {
    () => {
        Named!();
    };
}

macro_rules! impl_680 {
    () => {
        deps!();
        impl std :: ops :: Deref for Named < '_ > { type Target = CStr ; # [inline] fn deref (& self) -> & CStr { match self { Named :: Small (s) => s . as_cstr () , Named :: C (s) => s , } } }
    };
}

impl_680!()