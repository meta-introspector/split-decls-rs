macro_rules! deps {
    () => {
        SmallCString!();
    };
}

macro_rules! impl_667 {
    () => {
        deps!();
        impl std :: borrow :: Borrow < str > for SmallCString { # [inline] fn borrow (& self) -> & str { self . as_str () } }
    };
}

impl_667!();