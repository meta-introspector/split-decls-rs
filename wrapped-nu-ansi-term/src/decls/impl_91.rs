macro_rules! deps {
    () => {
        Rgb!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl core :: ops :: Mul < & f32 > for Rgb { type Output = Rgb ; fn mul (self , rhs : & f32) -> Self :: Output { rgb_mul_f32 (& self , rhs) } }
    };
}

impl_91!()