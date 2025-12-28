macro_rules! deps {
    () => {
        EcdsaCurve!();
        SignatureSize!();
        SigningKey!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl < C > ConstantTimeEq for SigningKey < C > where C : EcdsaCurve + CurveArithmetic , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn ct_eq (& self , other : & Self) -> Choice { self . secret_scalar . ct_eq (& other . secret_scalar) } }
    };
}

impl_77!();