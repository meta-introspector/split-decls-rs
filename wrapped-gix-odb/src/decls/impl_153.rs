macro_rules! deps {
    () => {
        Proxy!();
    };
}

macro_rules! impl_153 {
    () => {
        deps!();
        impl < T > Clone for Proxy < T > where T : Clone , { fn clone (& self) -> Self { Proxy { inner : self . inner . clone () , object_hash : self . object_hash , memory : self . memory . clone () , } } }
    };
}

impl_153!();