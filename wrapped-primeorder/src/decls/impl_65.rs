macro_rules! deps {
    () => {
        AffinePoint!();
        PrimeCurveParams!();
        ProjectivePoint!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl < C > FromEncodedPoint < C > for ProjectivePoint < C > where C : PrimeCurveParams , FieldBytes < C > : Copy , FieldBytesSize < C > : ModulusSize , CompressedPoint < C > : Copy , { fn from_encoded_point (p : & EncodedPoint < C >) -> CtOption < Self > { AffinePoint :: < C > :: from_encoded_point (p) . map (Self :: from) } }
    };
}

impl_65!();