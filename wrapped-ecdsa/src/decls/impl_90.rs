macro_rules! deps {
    () => {
        SigningKey!();
        EcdsaCurve!();
        SignatureSize!();
        VerifyingKey!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        # [cfg (feature = "algorithm")] impl < C > From < & SigningKey < C > > for VerifyingKey < C > where C : EcdsaCurve + CurveArithmetic , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn from (signing_key : & SigningKey < C >) -> VerifyingKey < C > { signing_key . verifying_key } }
    };
}

impl_90!()