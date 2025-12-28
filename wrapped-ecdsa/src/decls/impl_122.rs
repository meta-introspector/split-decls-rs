macro_rules! deps {
    () => {
        EcdsaCurve!();
        VerifyingKey!();
    };
}

macro_rules! impl_122 {
    () => {
        deps!();
        impl < C > PartialOrd for VerifyingKey < C > where C : EcdsaCurve + CurveArithmetic , AffinePoint < C > : FromEncodedPoint < C > + ToEncodedPoint < C > , FieldBytesSize < C > : sec1 :: ModulusSize , { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_122!()