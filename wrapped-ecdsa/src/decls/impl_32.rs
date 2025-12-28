macro_rules! deps {
    () => {
        EcdsaCurve!();
        MaxSize!();
        Signature!();
        MaxOverhead!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < C > SignatureBitStringEncoding for Signature < C > where C : EcdsaCurve , MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < MaxOverhead > + ArraySize , { fn to_bitstring (& self) -> der :: Result < BitString > { BitString :: new (0 , self . to_vec ()) } }
    };
}

impl_32!()