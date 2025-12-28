macro_rules! deps {
    () => {
        PrimeCurveParams!();
        AffinePoint!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < C > PartialEq for AffinePoint < C > where C : PrimeCurveParams , { fn eq (& self , other : & Self) -> bool { self . ct_eq (other) . into () } }
    };
}

impl_34!()