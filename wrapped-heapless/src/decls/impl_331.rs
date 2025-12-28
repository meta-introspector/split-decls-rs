macro_rules! deps {
    () => {
        VecInner!();
        LenType!();
    };
}

macro_rules! impl_331 {
    () => {
        deps!();
        impl < T , LenT : LenType , S : VecStorage < T > + ? Sized > AsMut < Self > for VecInner < T , LenT , S > { # [inline] fn as_mut (& mut self) -> & mut Self { self } }
    };
}

impl_331!();