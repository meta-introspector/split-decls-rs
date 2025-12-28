macro_rules! deps {
    () => {
        Difference!();
    };
}

macro_rules! impl_172 {
    () => {
        deps!();
        impl < 'a , T , S > Clone for Difference < 'a , T , S > { # [inline] fn clone (& self) -> Difference < 'a , T , S > { Difference { iter : self . iter . clone () , .. * self } } }
    };
}

impl_172!();