macro_rules! deps {
    () => {
        Signature!();
        EcdsaCurve!();
        SignatureSize!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < C > Copy for Signature < C > where C : EcdsaCurve , SignatureSize < C > : ArraySize , < SignatureSize < C > as ArraySize > :: ArrayType < u8 > : Copy , { }
    };
}

impl_21!()