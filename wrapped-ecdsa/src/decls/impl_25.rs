macro_rules! deps {
    () => {
        Signature!();
        EcdsaCurve!();
        MaxSize!();
        MaxOverhead!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < C > Encode for Signature < C > where C : EcdsaCurve , MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < MaxOverhead > + ArraySize , { fn encoded_len (& self) -> der :: Result < Length > { Length :: try_from (self . len ()) } fn encode (& self , writer : & mut impl Writer) -> der :: Result < () > { writer . write (self . as_bytes ()) } }
    };
}

impl_25!();