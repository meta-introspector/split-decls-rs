macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! impl_153 {
    () => {
        deps!();
        impl < T , const N : usize > BorrowMut < [T] > for SmallVec < T , N > { # [inline] fn borrow_mut (& mut self) -> & mut [T] { self . as_mut_slice () } }
    };
}

impl_153!();