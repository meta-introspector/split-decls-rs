macro_rules! deps {
    () => {
        VerifyingKey!();
        EcdsaCurve!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl < C > Ord for VerifyingKey < C > where C : EcdsaCurve + CurveArithmetic , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : sec1 :: ModulusSize , { fn cmp (& self , other : & Self) -> Ordering { self . inner . cmp (& other . inner) } }
    };
}

impl_123!();