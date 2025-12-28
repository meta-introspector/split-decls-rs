macro_rules! deps {
    () => {
        LenType!();
        VecInner!();
    };
}

macro_rules! impl_333 {
    () => {
        deps!();
        impl < T , LenT : LenType , S : VecStorage < T > + ? Sized > AsMut < [T] > for VecInner < T , LenT , S > { # [inline] fn as_mut (& mut self) -> & mut [T] { self } }
    };
}

impl_333!();