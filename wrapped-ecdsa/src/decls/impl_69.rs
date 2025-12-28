macro_rules! deps {
    () => {
        Signature!();
        MaxSize!();
        EcdsaCurve!();
        DigestAlgorithm!();
        SignatureSize!();
        SigningKey!();
        MaxOverhead!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        # [cfg (feature = "der")] impl < C > PrehashSigner < der :: Signature < C > > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , der :: MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < der :: MaxOverhead > + ArraySize , { fn sign_prehash (& self , prehash : & [u8]) -> Result < der :: Signature < C > > { PrehashSigner :: < Signature < C > > :: sign_prehash (self , prehash) . map (Into :: into) } }
    };
}

impl_69!();