macro_rules! deps {
    () => {
        SignatureWithOid!();
        SignatureBytes!();
        EcdsaCurve!();
        SignatureSize!();
    };
}

macro_rules! impl_165 {
    () => {
        deps!();
        # [cfg (feature = "digest")] impl < C > From < SignatureWithOid < C > > for SignatureBytes < C > where C : EcdsaCurve , SignatureSize < C > : ArraySize , { fn from (signature : SignatureWithOid < C >) -> SignatureBytes < C > { signature . to_bytes () } }
    };
}

impl_165!();