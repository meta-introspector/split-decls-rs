macro_rules! deps {
    () => {
        SigningKey!();
        EcdsaCurve!();
        SignatureWithOid!();
        SignatureSize!();
        Signature!();
        DigestAlgorithm!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl < C , D > DigestSigner < D , SignatureWithOid < C > > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , D : AssociatedOid + EagerHash + Update , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn try_sign_digest < F : Fn (& mut D) -> Result < () > > (& self , f : F) -> Result < SignatureWithOid < C > > { let signature : Signature < C > = self . try_sign_digest (f) ? ; let oid = ecdsa_oid_for_digest (D :: OID) . ok_or_else (Error :: new) ? ; SignatureWithOid :: new (signature , oid) } }
    };
}

impl_66!()