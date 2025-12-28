macro_rules! deps {
    () => {
        VerifyingKey!();
        Signature!();
        SignatureSize!();
        EcdsaCurve!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        impl < C > PrehashVerifier < Signature < C > > for VerifyingKey < C > where C : EcdsaCurve + CurveArithmetic , SignatureSize < C > : ArraySize , { fn verify_prehash (& self , prehash : & [u8] , signature : & Signature < C >) -> Result < () > { if C :: NORMALIZE_S && signature . s () . is_high () . into () { return Err (Error :: new ()) ; } hazmat :: verify_prehashed :: < C > (& ProjectivePoint :: < C > :: from (* self . inner . as_affine ()) , & bits2field :: < C > (prehash) ? , signature ,) } }
    };
}

impl_101!()