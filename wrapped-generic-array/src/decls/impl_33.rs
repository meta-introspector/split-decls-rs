macro_rules! deps {
    () => {
        ArrayLength!();
        GenericArray!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < T , N : ArrayLength > BorrowMut < [T] > for GenericArray < T , N > { # [inline (always)] fn borrow_mut (& mut self) -> & mut [T] { self . as_mut_slice () } }
    };
}

impl_33!()