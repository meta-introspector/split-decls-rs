macro_rules! deps {
    () => {
        Rgb!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl core :: ops :: Add < Rgb > for & Rgb { type Output = Rgb ; fn add (self , rhs : Rgb) -> Self :: Output { rgb_add (self , & rhs) } }
    };
}

impl_84!();