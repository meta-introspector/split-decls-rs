macro_rules! deps {
    () => {
        ProjectivePoint!();
        PrimeCurveParams!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        impl < C > ConstantTimeEq for ProjectivePoint < C > where C : PrimeCurveParams , { fn ct_eq (& self , other : & Self) -> Choice { self . to_affine () . ct_eq (& other . to_affine ()) } }
    };
}

impl_55!();