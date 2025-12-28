macro_rules! deps {
    () => {
        Frozen!();
    };
}

macro_rules! impl_760 {
    () => {
        deps!();
        impl < G , I > Index < I > for Frozen < '_ , G > where G : Index < I > , { type Output = G :: Output ; fn index (& self , i : I) -> & G :: Output { self . 0 . index (i) } }
    };
}

impl_760!()