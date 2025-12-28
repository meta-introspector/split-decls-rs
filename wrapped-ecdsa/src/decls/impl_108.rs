macro_rules! deps {
    () => {
        SignatureSize!();
        MaxOverhead!();
        Signature!();
        VerifyingKey!();
        EcdsaCurve!();
        DigestAlgorithm!();
        MaxSize!();
    };
}

macro_rules! impl_108 {
    () => {
        deps!();
        # [cfg (feature = "der")] impl < C > Verifier < der :: Signature < C > > for VerifyingKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , SignatureSize < C > : ArraySize , der :: MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < der :: MaxOverhead > + ArraySize , { fn verify (& self , msg : & [u8] , signature : & der :: Signature < C >) -> Result < () > { let signature = Signature :: < C > :: try_from (signature . clone ()) ? ; Verifier :: < Signature < C > > :: verify (self , msg , & signature) } }
    };
}

impl_108!();