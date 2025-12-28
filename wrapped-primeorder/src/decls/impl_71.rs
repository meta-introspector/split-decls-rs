macro_rules! deps {
    () => {
        PrimeCurveParams!();
        ProjectivePoint!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl < C > PrimeGroup for ProjectivePoint < C > where Self : Double , C : PrimeCurveParams , CompressedPoint < C > : Copy + Send + Sync , FieldBytes < C > : Copy , FieldBytesSize < C > : ModulusSize , < UncompressedPointSize < C > as ArraySize > :: ArrayType < u8 > : Copy , { }
    };
}

impl_71!()