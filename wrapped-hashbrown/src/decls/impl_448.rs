macro_rules! deps {
    () => {
        SymmetricDifference!();
    };
}

macro_rules! impl_448 {
    () => {
        deps!();
        impl < T , S , A : Allocator > Clone for SymmetricDifference < '_ , T , S , A > { # [cfg_attr (feature = "inline-more" , inline)] fn clone (& self) -> Self { SymmetricDifference { iter : self . iter . clone () , } } }
    };
}

impl_448!()