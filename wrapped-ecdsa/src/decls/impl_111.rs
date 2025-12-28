macro_rules! deps {
    () => {
        EcdsaCurve!();
        VerifyingKey!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        impl < C > Copy for VerifyingKey < C > where C : EcdsaCurve + CurveArithmetic { }
    };
}

impl_111!();