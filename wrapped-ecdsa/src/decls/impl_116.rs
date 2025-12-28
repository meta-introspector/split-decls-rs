macro_rules! deps {
    () => {
        EcdsaCurve!();
        VerifyingKey!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        impl < C > Eq for VerifyingKey < C > where C : EcdsaCurve + CurveArithmetic { }
    };
}

impl_116!()