macro_rules! deps {
    () => {
        Str!();
    };
}

macro_rules! impl_206 {
    () => {
        deps!();
        impl std :: ops :: Deref for Str { type Target = str ; # [inline] fn deref (& self) -> & str { self . as_str () } }
    };
}

impl_206!();