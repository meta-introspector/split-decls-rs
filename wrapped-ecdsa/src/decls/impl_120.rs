macro_rules! deps {
    () => {
        VerifyingKey!();
        EcdsaCurve!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl < C > From < VerifyingKey < C > > for PublicKey < C > where C : EcdsaCurve + CurveArithmetic , { fn from (verifying_key : VerifyingKey < C >) -> PublicKey < C > { verifying_key . inner } }
    };
}

impl_120!()