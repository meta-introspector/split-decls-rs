macro_rules! deps {
    () => {
        PropName!();
        PropertyName!();
    };
}

macro_rules! impl_319 {
    () => {
        deps!();
        impl std :: borrow :: Borrow < PropName > for PropertyName { # [inline] fn borrow (& self) -> & PropName { self } }
    };
}

impl_319!()