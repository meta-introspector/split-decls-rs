macro_rules! deps {
    () => {
        MaxOverhead!();
        MaxSize!();
        Signature!();
        EcdsaCurve!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < C > SignatureEncoding for Signature < C > where C : EcdsaCurve , MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < MaxOverhead > + ArraySize , { type Repr = Box < [u8] > ; fn to_vec (& self) -> Vec < u8 > { self . as_bytes () . into () } }
    };
}

impl_31!()