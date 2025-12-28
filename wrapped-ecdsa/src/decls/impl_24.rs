macro_rules! deps {
    () => {
        Signature!();
        EcdsaCurve!();
        SignatureSize!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < C > TryFrom < & [u8] > for Signature < C > where C : EcdsaCurve , SignatureSize < C > : ArraySize , { type Error = Error ; fn try_from (slice : & [u8]) -> Result < Self > { Self :: from_slice (slice) } }
    };
}

impl_24!()