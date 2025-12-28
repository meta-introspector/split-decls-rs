macro_rules! deps {
    () => {
        Rgb!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl core :: ops :: Neg for Rgb { type Output = Rgb ; fn neg (self) -> Self :: Output { rgb_negate (& self) } }
    };
}

impl_98!();