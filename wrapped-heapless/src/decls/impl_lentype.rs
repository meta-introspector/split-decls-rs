macro_rules! deps {
    () => {
        LenType!();
        Sealed!();
    };
}

macro_rules! impl_lentype {
    () => {
        deps!();
        macro_rules ! impl_lentype { ($ ($ (# [$ meta : meta]) * $ LenT : ty) ,*) => { $ ($ (# [$ meta]) * impl Sealed for $ LenT { const ZERO : Self = 0 ; const MAX : Self = Self :: MAX ; const MAX_USIZE : usize = Self :: MAX as _ ; fn one () -> Self { 1 } } $ (# [$ meta]) * impl LenType for $ LenT { }) * } }
    };
}

impl_lentype!()