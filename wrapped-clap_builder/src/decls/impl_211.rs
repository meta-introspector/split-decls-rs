macro_rules! deps {
    () => {
        Str!();
    };
}

macro_rules! impl_211 {
    () => {
        deps!();
        impl std :: borrow :: Borrow < str > for Str { # [inline] fn borrow (& self) -> & str { self . as_str () } }
    };
}

impl_211!()