macro_rules! deps {
    () => {
        Rgb!();
    };
}

macro_rules! impl_86 {
    () => {
        deps!();
        impl core :: ops :: Sub < Rgb > for Rgb { type Output = Rgb ; fn sub (self , rhs : Rgb) -> Self :: Output { rgb_sub (& self , & rhs) } }
    };
}

impl_86!();