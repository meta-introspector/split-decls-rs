macro_rules! deps {
    () => {
        LenType!();
        VecInner!();
    };
}

macro_rules! impl_328 {
    () => {
        deps!();
        impl < T , LenT : LenType , S : VecStorage < T > + ? Sized > borrow :: Borrow < [T] > for VecInner < T , LenT , S > { fn borrow (& self) -> & [T] { self . as_slice () } }
    };
}

impl_328!();