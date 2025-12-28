macro_rules! deps {
    () => {
        EcdsaCurve!();
        Signature!();
        MaxOverhead!();
        MaxSize!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < C > Debug for Signature < C > where C : EcdsaCurve , MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < MaxOverhead > + ArraySize , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "ecdsa::der::Signature<{:?}>(" , C :: default ()) ? ; for & byte in self . as_ref () { write ! (f , "{byte:02X}") ? ; } write ! (f , ")") } }
    };
}

impl_23!()