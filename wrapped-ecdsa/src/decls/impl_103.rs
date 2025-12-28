macro_rules! deps {
    () => {
        VerifyingKey!();
        DigestAlgorithm!();
        Signature!();
        SignatureSize!();
        EcdsaCurve!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl < C > MultipartVerifier < Signature < C > > for VerifyingKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , SignatureSize < C > : ArraySize , { fn multipart_verify (& self , msg : & [& [u8]] , signature : & Signature < C >) -> Result < () > { self . verify_digest (| digest : & mut C :: Digest | { msg . iter () . for_each (| slice | digest . update (slice)) ; Ok (()) } , signature ,) } }
    };
}

impl_103!()