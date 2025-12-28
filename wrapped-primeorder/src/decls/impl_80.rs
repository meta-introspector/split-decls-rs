macro_rules! deps {
    () => {
        ProjectivePoint!();
        PrimeCurveParams!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        impl < C > PartialEq for ProjectivePoint < C > where C : PrimeCurveParams , { fn eq (& self , other : & Self) -> bool { self . ct_eq (other) . into () } }
    };
}

impl_80!();