macro_rules! deps {
    () => {
        ArrayVec!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl < T , const CAP : usize > BorrowMut < [T] > for ArrayVec < T , CAP > { fn borrow_mut (& mut self) -> & mut [T] { self } }
    };
}

impl_78!()