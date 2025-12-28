macro_rules! deps {
    () => {
        Float!();
        Kernel!();
        Gaussian!();
    };
}

macro_rules! impl_343 {
    () => {
        deps!();
        impl < A > Kernel < A > for Gaussian where A : Float , { fn evaluate (& self , x : A) -> A { use std :: f32 :: consts :: PI ; (x . powi (2) . exp () * A :: cast (2. * PI)) . sqrt () . recip () } }
    };
}

impl_343!();