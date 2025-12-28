macro_rules! deps {
    () => {
        Split!();
    };
}

macro_rules! impl_1263 {
    () => {
        deps!();
        impl < T , P : Clone > Clone for Split < '_ , T , P > { fn clone (& self) -> Self { Split { separator : self . separator . clone () , .. * self } } }
    };
}

impl_1263!();