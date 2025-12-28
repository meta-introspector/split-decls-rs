macro_rules! deps {
    () => {
        DigestAlgorithm!();
        EcdsaCurve!();
        Signature!();
        RecoveryId!();
        SignatureSize!();
        SigningKey!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        # [cfg (feature = "algorithm")] impl < C > Signer < (Signature < C > , RecoveryId) > for SigningKey < C > where C : EcdsaCurve + CurveArithmetic + DigestAlgorithm , Scalar < C > : Invert < Output = CtOption < Scalar < C > > > , SignatureSize < C > : ArraySize , { fn try_sign (& self , msg : & [u8]) -> Result < (Signature < C > , RecoveryId) > { self . try_multipart_sign (& [msg]) } }
    };
}

impl_11!()