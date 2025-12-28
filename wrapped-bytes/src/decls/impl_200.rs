macro_rules! deps {
    () => {
        BytesMut!();
    };
}

macro_rules! impl_200 {
    () => {
        deps!();
        impl BorrowMut < [u8] > for BytesMut { fn borrow_mut (& mut self) -> & mut [u8] { self . as_mut () } }
    };
}

impl_200!()