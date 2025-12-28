macro_rules! deps {
    () => {
        MaxSize!();
        Signature!();
        EcdsaCurve!();
        MaxOverhead!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < C > TryFrom < & [u8] > for Signature < C > where C : EcdsaCurve , MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < MaxOverhead > + ArraySize , { type Error = Error ; fn try_from (input : & [u8]) -> Result < Self > { Self :: from_bytes (input) } }
    };
}

impl_28!();