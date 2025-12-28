macro_rules! deps {
    () => {
        VerifyingKey!();
        MaxOverhead!();
        MaxSize!();
        EcdsaCurve!();
        Signature!();
        SignatureSize!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        # [cfg (feature = "der")] impl < C , D > DigestVerifier < D , der :: Signature < C > > for VerifyingKey < C > where C : EcdsaCurve + CurveArithmetic , D : EagerHash + Update , SignatureSize < C > : ArraySize , der :: MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < der :: MaxOverhead > + ArraySize , { fn verify_digest < F : Fn (& mut D) -> Result < () > > (& self , f : F , signature : & der :: Signature < C > ,) -> Result < () > { let signature = Signature :: < C > :: try_from (signature . clone ()) ? ; DigestVerifier :: < D , Signature < C > > :: verify_digest (self , f , & signature) } }
    };
}

impl_106!()