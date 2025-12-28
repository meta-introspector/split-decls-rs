macro_rules! deps {
    () => {
        EcdsaCurve!();
        SigningKey!();
        SignatureSize!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl < C > PartialEq for SigningKey < C > where C : EcdsaCurve + CurveArithmetic , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn eq (& self , other : & SigningKey < C >) -> bool { self . ct_eq (other) . into () } }
    };
}

impl_81!()