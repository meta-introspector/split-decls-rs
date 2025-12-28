macro_rules! deps {
    () => {
        Signature!();
        EcdsaCurve!();
        SignatureWithOid!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        # [cfg (feature = "digest")] impl < C > From < SignatureWithOid < C > > for Signature < C > where C : EcdsaCurve , { fn from (sig : SignatureWithOid < C >) -> Signature < C > { sig . signature } }
    };
}

impl_38!()