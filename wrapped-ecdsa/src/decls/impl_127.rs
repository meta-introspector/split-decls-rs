macro_rules! deps {
    () => {
        EcdsaCurve!();
        VerifyingKey!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        # [cfg (feature = "pkcs8")] impl < C > TryFrom < pkcs8 :: SubjectPublicKeyInfoRef < '_ > > for VerifyingKey < C > where C : EcdsaCurve + AssociatedOid + CurveArithmetic + PointCompression , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : sec1 :: ModulusSize , { type Error = spki :: Error ; fn try_from (spki : pkcs8 :: SubjectPublicKeyInfoRef < '_ >) -> spki :: Result < Self > { PublicKey :: try_from (spki) . map (| inner | Self { inner }) } }
    };
}

impl_127!()