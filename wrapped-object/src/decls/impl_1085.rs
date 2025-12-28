macro_rules! deps {
    () => {
        ByteString!();
    };
}

macro_rules! impl_1085 {
    () => {
        deps!();
        impl < 'a > core :: borrow :: Borrow < [u8] > for ByteString < 'a > { fn borrow (& self) -> & [u8] { self . 0 . borrow () } }
    };
}

impl_1085!()