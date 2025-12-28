macro_rules! deps {
    () => {
        Signature!();
        SignatureSize!();
        EcdsaCurve!();
        SignatureBytes!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl < C > SignatureEncoding for Signature < C > where C : EcdsaCurve , SignatureSize < C > : ArraySize , { type Repr = SignatureBytes < C > ; }
    };
}

impl_149!()