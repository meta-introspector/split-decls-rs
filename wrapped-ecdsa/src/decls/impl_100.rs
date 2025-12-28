macro_rules! deps {
    () => {
        SignatureSize!();
        Signature!();
        VerifyingKey!();
        EcdsaCurve!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl < C , D > DigestVerifier < D , Signature < C > > for VerifyingKey < C > where C : EcdsaCurve + CurveArithmetic , D : EagerHash + Update , SignatureSize < C > : ArraySize , { fn verify_digest < F : Fn (& mut D) -> Result < () > > (& self , f : F , signature : & Signature < C > ,) -> Result < () > { let mut digest = D :: new () ; f (& mut digest) ? ; self . verify_prehash (& digest . finalize () , signature) } }
    };
}

impl_100!()