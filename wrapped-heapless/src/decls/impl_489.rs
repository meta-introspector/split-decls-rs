macro_rules! deps {
    () => {
        LenType!();
        VecInner!();
        CapacityError!();
    };
}

macro_rules! impl_489 {
    () => {
        deps!();
        impl < LenT : LenType , S : VecStorage < u8 > + ? Sized > ErrorType for VecInner < u8 , LenT , S > { type Error = CapacityError ; }
    };
}

impl_489!()