macro_rules! deps {
    () => {
        SignatureWithOid!();
        VerifyingKey!();
        EcdsaCurve!();
        DigestAlgorithm!();
        SignatureSize!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        # [cfg (feature = "sha2")] impl < C > Verifier < SignatureWithOid < C > > for VerifyingKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , SignatureSize < C > : ArraySize , { fn verify (& self , msg : & [u8] , sig : & SignatureWithOid < C >) -> Result < () > { self . multipart_verify (& [msg] , sig) } }
    };
}

impl_104!();