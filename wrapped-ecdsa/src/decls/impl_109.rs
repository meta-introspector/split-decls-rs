macro_rules! deps {
    () => {
        EcdsaCurve!();
        DigestAlgorithm!();
        MaxOverhead!();
        SignatureSize!();
        Signature!();
        VerifyingKey!();
        MaxSize!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        # [cfg (feature = "der")] impl < C > MultipartVerifier < der :: Signature < C > > for VerifyingKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , SignatureSize < C > : ArraySize , der :: MaxSize < C > : ArraySize , < FieldBytesSize < C > as Add > :: Output : Add < der :: MaxOverhead > + ArraySize , { fn multipart_verify (& self , msg : & [& [u8]] , signature : & der :: Signature < C >) -> Result < () > { let signature = Signature :: < C > :: try_from (signature . clone ()) ? ; MultipartVerifier :: < Signature < C > > :: multipart_verify (self , msg , & signature) } }
    };
}

impl_109!();