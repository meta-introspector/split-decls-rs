macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_423 {
    () => {
        deps!();
        impl < K > Clone for Iter < '_ , K > { # [cfg_attr (feature = "inline-more" , inline)] fn clone (& self) -> Self { Iter { iter : self . iter . clone () , } } }
    };
}

impl_423!();