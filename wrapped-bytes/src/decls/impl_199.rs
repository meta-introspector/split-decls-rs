macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_199 {
    () => {
        deps!();
        impl Borrow < [u8] > for BytesMut { fn borrow (& self) -> & [u8] { self . as_ref () } }
    };
}

impl_199!()