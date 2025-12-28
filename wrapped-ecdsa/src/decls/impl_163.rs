macro_rules! deps {
    () => {
        SignatureWithOid!();
        EcdsaCurve!();
        SignatureSize!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        # [cfg (feature = "digest")] impl < C > Copy for SignatureWithOid < C > where C : EcdsaCurve , SignatureSize < C > : ArraySize , < SignatureSize < C > as ArraySize > :: ArrayType < u8 > : Copy , { }
    };
}

impl_163!();