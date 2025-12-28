macro_rules! deps {
    () => {
        VecInner!();
        LenType!();
    };
}

macro_rules! impl_329 {
    () => {
        deps!();
        impl < T , LenT : LenType , S : VecStorage < T > + ? Sized > borrow :: BorrowMut < [T] > for VecInner < T , LenT , S > { fn borrow_mut (& mut self) -> & mut [T] { self . as_mut_slice () } }
    };
}

impl_329!();