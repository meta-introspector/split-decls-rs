macro_rules! deps {
    () => {
        Positioned!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl BorrowMut < str > for Positioned < String > { fn borrow_mut (& mut self) -> & mut str { self . node . as_mut_str () } }
    };
}

impl_130!();