macro_rules! deps {
    () => {
        Difference!();
    };
}

macro_rules! impl_444 {
    () => {
        deps!();
        impl < T , S , A : Allocator > Clone for Difference < '_ , T , S , A > { # [cfg_attr (feature = "inline-more" , inline)] fn clone (& self) -> Self { Difference { iter : self . iter . clone () , .. * self } } }
    };
}

impl_444!()