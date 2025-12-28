macro_rules! deps {
    () => {
        SymmetricDifference!();
    };
}

macro_rules! impl_175 {
    () => {
        deps!();
        impl < 'a , T , S > Clone for SymmetricDifference < 'a , T , S > { # [inline] fn clone (& self) -> SymmetricDifference < 'a , T , S > { SymmetricDifference { iter : self . iter . clone () , } } }
    };
}

impl_175!()