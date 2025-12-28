macro_rules! deps {
    () => {
        VerifyingKey!();
        EcdsaCurve!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl < C > From < & PublicKey < C > > for VerifyingKey < C > where C : EcdsaCurve + CurveArithmetic , { fn from (public_key : & PublicKey < C >) -> VerifyingKey < C > { (* public_key) . into () } }
    };
}

impl_119!();