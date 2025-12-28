macro_rules! deps {
    () => {
        PrimeCurveParams!();
        AffinePoint!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < C > From < AffinePoint < C > > for EncodedPoint < C > where C : PrimeCurveParams , FieldBytesSize < C > : ModulusSize , CompressedPoint < C > : Copy , < UncompressedPointSize < C > as ArraySize > :: ArrayType < u8 > : Copy , { fn from (affine : AffinePoint < C >) -> EncodedPoint < C > { affine . to_encoded_point (false) } }
    };
}

impl_32!();