macro_rules! deps {
    () => {
        PrimeCurveParams!();
        ProjectivePoint!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl < C > ToEncodedPoint < C > for ProjectivePoint < C > where C : PrimeCurveParams , CompressedPoint < C > : Copy , FieldBytesSize < C > : ModulusSize , < UncompressedPointSize < C > as ArraySize > :: ArrayType < u8 > : Copy , { fn to_encoded_point (& self , compress : bool) -> EncodedPoint < C > { self . to_affine () . to_encoded_point (compress) } }
    };
}

impl_81!();