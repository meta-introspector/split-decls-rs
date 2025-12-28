macro_rules! deps {
    () => {
        Intersection!();
    };
}

macro_rules! impl_440 {
    () => {
        deps!();
        impl < T , S , A : Allocator > Clone for Intersection < '_ , T , S , A > { # [cfg_attr (feature = "inline-more" , inline)] fn clone (& self) -> Self { Intersection { iter : self . iter . clone () , .. * self } } }
    };
}

impl_440!();