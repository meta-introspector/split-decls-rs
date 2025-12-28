macro_rules! deps {
    () => {
        Signature!();
        EcdsaCurve!();
        SignatureSize!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < C > fmt :: Display for Signature < C > where C : EcdsaCurve , SignatureSize < C > : ArraySize , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{self:X}") } }
    };
}

impl_26!()