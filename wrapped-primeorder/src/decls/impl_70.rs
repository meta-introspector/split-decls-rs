macro_rules! deps {
    () => {
        AffinePoint!();
        ProjectivePoint!();
        PrimeCurveParams!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl < C > PrimeCurve for ProjectivePoint < C > where Self : Double , C : PrimeCurveParams , CompressedPoint < C > : Copy + Send + Sync , FieldBytes < C > : Copy , FieldBytesSize < C > : ModulusSize , < UncompressedPointSize < C > as ArraySize > :: ArrayType < u8 > : Copy , { type Affine = AffinePoint < C > ; }
    };
}

impl_70!()