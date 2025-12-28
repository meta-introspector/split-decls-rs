macro_rules! deps {
    () => {
        SignatureSize!();
        EcdsaCurve!();
        Signature!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl < C > TryFrom < & [u8] > for Signature < C > where C : EcdsaCurve , SignatureSize < C > : ArraySize , { type Error = Error ; fn try_from (slice : & [u8]) -> Result < Self > { Self :: from_slice (slice) } }
    };
}

impl_150!();