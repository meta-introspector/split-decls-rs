macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl Borrow < [u8] > for Bytes { fn borrow (& self) -> & [u8] { self . as_slice () } }
    };
}

impl_76!();