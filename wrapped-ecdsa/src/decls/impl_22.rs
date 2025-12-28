macro_rules! deps {
    () => {
        SignatureSize!();
        EcdsaCurve!();
        Signature!();
        SignatureBytes!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < C > From < Signature < C > > for SignatureBytes < C > where C : EcdsaCurve , SignatureSize < C > : ArraySize , { fn from (signature : Signature < C >) -> SignatureBytes < C > { signature . to_bytes () } }
    };
}

impl_22!()