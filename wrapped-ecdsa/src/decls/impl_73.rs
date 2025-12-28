macro_rules! deps {
    () => {
        EcdsaCurve!();
        MaxSize!();
        SignatureSize!();
        Signature!();
        DigestAlgorithm!();
        MaxOverhead!();
        SigningKey!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        # [cfg (feature = "der")] impl < D , C > DigestSigner < D , der :: Signature < C > > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , D : EagerHash + Update , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , der :: MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < der :: MaxOverhead > + ArraySize , { fn try_sign_digest < F : Fn (& mut D) -> Result < () > > (& self , f : F) -> Result < der :: Signature < C > > { DigestSigner :: < D , Signature < C > > :: try_sign_digest (self , f) . map (Into :: into) } }
    };
}

impl_73!();