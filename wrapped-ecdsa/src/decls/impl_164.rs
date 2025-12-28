macro_rules! deps {
    () => {
        EcdsaCurve!();
        SignatureWithOid!();
        Signature!();
    };
}

macro_rules! impl_164 {
    () => {
        deps!();
        # [cfg (feature = "digest")] impl < C > From < SignatureWithOid < C > > for Signature < C > where C : EcdsaCurve , { fn from (sig : SignatureWithOid < C >) -> Signature < C > { sig . signature } }
    };
}

impl_164!();