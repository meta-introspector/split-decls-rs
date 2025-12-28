macro_rules! deps {
    () => {
        Positioned!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl Borrow < str > for Positioned < String > { fn borrow (& self) -> & str { self . node . as_str () } }
    };
}

impl_129!();