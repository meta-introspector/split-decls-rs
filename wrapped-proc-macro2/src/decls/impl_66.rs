macro_rules! deps {
    () => {
        RcVec!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl < T > Clone for RcVec < T > { fn clone (& self) -> Self { RcVec { inner : Rc :: clone (& self . inner) , } } }
    };
}

impl_66!()