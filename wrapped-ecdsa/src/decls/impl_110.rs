macro_rules! deps {
    () => {
        EcdsaCurve!();
        VerifyingKey!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl < C > AsRef < AffinePoint < C > > for VerifyingKey < C > where C : EcdsaCurve + CurveArithmetic , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : sec1 :: ModulusSize , { fn as_ref (& self) -> & AffinePoint < C > { self . as_affine () } }
    };
}

impl_110!()