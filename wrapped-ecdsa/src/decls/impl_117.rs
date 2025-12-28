macro_rules! deps {
    () => {
        EcdsaCurve!();
        VerifyingKey!();
    };
}

macro_rules! impl_117 {
    () => {
        deps!();
        impl < C > PartialEq for VerifyingKey < C > where C : EcdsaCurve + CurveArithmetic , { fn eq (& self , other : & Self) -> bool { self . inner . eq (& other . inner) } }
    };
}

impl_117!()