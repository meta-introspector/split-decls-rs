macro_rules! deps {
    () => {
        Id!();
    };
}

macro_rules! impl_658 {
    () => {
        deps!();
        impl std :: borrow :: Borrow < str > for Id { # [inline] fn borrow (& self) -> & str { self . as_str () } }
    };
}

impl_658!()