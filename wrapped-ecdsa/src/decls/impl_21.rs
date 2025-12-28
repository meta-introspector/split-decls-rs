macro_rules! deps {
    () => {
        Signature!();
        MaxSize!();
        EcdsaCurve!();
        MaxOverhead!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < C > AsRef < [u8] > for Signature < C > where C : EcdsaCurve , MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < MaxOverhead > + ArraySize , { fn as_ref (& self) -> & [u8] { self . as_bytes () } }
    };
}

impl_21!();