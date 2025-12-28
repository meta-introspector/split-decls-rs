macro_rules! deps {
    () => {
        SignatureSize!();
        VerifyingKey!();
        EcdsaCurve!();
        SigningKey!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        # [cfg (feature = "algorithm")] impl < C > AsRef < VerifyingKey < C > > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn as_ref (& self) -> & VerifyingKey < C > { & self . verifying_key } }
    };
}

impl_76!()