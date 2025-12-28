macro_rules! deps {
    () => {
        EcdsaCurve!();
        VerifyingKey!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl < C > From < & VerifyingKey < C > > for PublicKey < C > where C : EcdsaCurve + CurveArithmetic , { fn from (verifying_key : & VerifyingKey < C >) -> PublicKey < C > { (* verifying_key) . into () } }
    };
}

impl_121!()