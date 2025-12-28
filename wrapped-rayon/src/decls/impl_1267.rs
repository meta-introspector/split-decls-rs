macro_rules! deps {
    () => {
        SplitInclusive!();
    };
}

macro_rules! impl_1267 {
    () => {
        deps!();
        impl < T , P : Clone > Clone for SplitInclusive < '_ , T , P > { fn clone (& self) -> Self { SplitInclusive { separator : self . separator . clone () , .. * self } } }
    };
}

impl_1267!();