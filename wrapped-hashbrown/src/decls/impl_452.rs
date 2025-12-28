macro_rules! deps {
    () => {
        Union!();
    };
}

macro_rules! impl_452 {
    () => {
        deps!();
        impl < T , S , A : Allocator > Clone for Union < '_ , T , S , A > { # [cfg_attr (feature = "inline-more" , inline)] fn clone (& self) -> Self { Union { iter : self . iter . clone () , } } }
    };
}

impl_452!();