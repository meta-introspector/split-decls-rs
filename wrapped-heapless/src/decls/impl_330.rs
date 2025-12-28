macro_rules! deps {
    () => {
        LenType!();
        VecInner!();
    };
}

macro_rules! impl_330 {
    () => {
        deps!();
        impl < T , LenT : LenType , S : VecStorage < T > + ? Sized > AsRef < Self > for VecInner < T , LenT , S > { # [inline] fn as_ref (& self) -> & Self { self } }
    };
}

impl_330!()