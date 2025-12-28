macro_rules! deps {
    () => {
        EcdsaCurve!();
        Signature!();
        SignatureBytes!();
        SignatureSize!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        impl < C > From < Signature < C > > for SignatureBytes < C > where C : EcdsaCurve , SignatureSize < C > : ArraySize , { fn from (signature : Signature < C >) -> SignatureBytes < C > { signature . to_bytes () } }
    };
}

impl_148!();