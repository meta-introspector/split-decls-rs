macro_rules! deps {
    () => {
        Signature!();
        SignatureSize!();
        EcdsaCurve!();
        MaxOverhead!();
        MaxSize!();
        SigningKey!();
        DigestAlgorithm!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        # [cfg (feature = "der")] impl < C > Signer < der :: Signature < C > > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , der :: MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < der :: MaxOverhead > + ArraySize , { fn try_sign (& self , msg : & [u8]) -> Result < der :: Signature < C > > { Signer :: < Signature < C > > :: try_sign (self , msg) . map (Into :: into) } }
    };
}

impl_70!();