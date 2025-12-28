macro_rules! deps {
    () => {
        EcdsaCurve!();
        SignatureSize!();
        Signature!();
        SignatureBytes!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < C > SignatureEncoding for Signature < C > where C : EcdsaCurve , SignatureSize < C > : ArraySize , { type Repr = SignatureBytes < C > ; }
    };
}

impl_23!()