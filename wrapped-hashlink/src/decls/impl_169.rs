macro_rules! deps {
    () => {
        Intersection!();
    };
}

macro_rules! impl_169 {
    () => {
        deps!();
        impl < 'a , T , S > Clone for Intersection < 'a , T , S > { # [inline] fn clone (& self) -> Intersection < 'a , T , S > { Intersection { iter : self . iter . clone () , .. * self } } }
    };
}

impl_169!();