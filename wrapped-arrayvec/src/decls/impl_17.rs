macro_rules! deps {
    () => {
        ArrayString!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl < const CAP : usize > BorrowMut < str > for ArrayString < CAP > { fn borrow_mut (& mut self) -> & mut str { self } }
    };
}

impl_17!()