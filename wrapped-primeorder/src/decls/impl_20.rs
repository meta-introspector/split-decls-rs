macro_rules! deps {
    () => {
        PrimeCurveParams!();
        AffinePoint!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < C > ConstantTimeEq for AffinePoint < C > where C : PrimeCurveParams , { fn ct_eq (& self , other : & Self) -> Choice { self . x . ct_eq (& other . x) & self . y . ct_eq (& other . y) & self . infinity . ct_eq (& other . infinity) } }
    };
}

impl_20!()