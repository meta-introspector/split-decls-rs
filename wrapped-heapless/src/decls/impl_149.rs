macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl < T > Clone for Iter < '_ , T > { fn clone (& self) -> Self { Self { iter : self . iter . clone () , } } }
    };
}

impl_149!()