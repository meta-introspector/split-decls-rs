macro_rules! deps {
    () => {
        SignatureSize!();
        Signature!();
        EcdsaCurve!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < C > fmt :: UpperHex for Signature < C > where C : EcdsaCurve , SignatureSize < C > : ArraySize , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { for byte in self . to_bytes () { write ! (f , "{byte:02X}") ? ; } Ok (()) } }
    };
}

impl_28!()