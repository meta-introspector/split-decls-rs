macro_rules! deps {
    () => {
        SignatureSize!();
        EcdsaCurve!();
        DigestAlgorithm!();
        Signature!();
        VerifyingKey!();
    };
}

macro_rules! impl_102 {
    () => {
        deps!();
        impl < C > Verifier < Signature < C > > for VerifyingKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , SignatureSize < C > : ArraySize , { fn verify (& self , msg : & [u8] , signature : & Signature < C >) -> Result < () > { self . multipart_verify (& [msg] , signature) } }
    };
}

impl_102!()