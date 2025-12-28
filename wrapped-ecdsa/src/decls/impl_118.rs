macro_rules! deps {
    () => {
        VerifyingKey!();
        EcdsaCurve!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl < C > From < PublicKey < C > > for VerifyingKey < C > where C : EcdsaCurve + CurveArithmetic , { fn from (public_key : PublicKey < C >) -> VerifyingKey < C > { VerifyingKey { inner : public_key } } }
    };
}

impl_118!()