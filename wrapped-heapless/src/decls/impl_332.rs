macro_rules! deps {
    () => {
        VecInner!();
        LenType!();
    };
}

macro_rules! impl_332 {
    () => {
        deps!();
        impl < T , LenT : LenType , S : VecStorage < T > + ? Sized > AsRef < [T] > for VecInner < T , LenT , S > { # [inline] fn as_ref (& self) -> & [T] { self } }
    };
}

impl_332!();