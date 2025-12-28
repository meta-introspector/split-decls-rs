macro_rules! deps {
    () => {
        Rgb!();
    };
}

macro_rules! impl_94 {
    () => {
        deps!();
        impl core :: ops :: Mul < Rgb > for f32 { type Output = Rgb ; fn mul (self , rhs : Rgb) -> Self :: Output { rgb_mul_f32 (& rhs , & self) } }
    };
}

impl_94!();