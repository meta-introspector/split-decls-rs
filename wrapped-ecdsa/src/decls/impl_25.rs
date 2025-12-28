macro_rules! deps {
    () => {
        EcdsaCurve!();
        Signature!();
        SignatureSize!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl < C > fmt :: Debug for Signature < C > where C : EcdsaCurve , SignatureSize < C > : ArraySize , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "ecdsa::Signature<{:?}>(" , C :: default ()) ? ; for byte in self . to_bytes () { write ! (f , "{byte:02X}") ? ; } write ! (f , ")") } }
    };
}

impl_25!()