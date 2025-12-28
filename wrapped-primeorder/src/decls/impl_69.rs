macro_rules! deps {
    () => {
        PrimeCurveParams!();
        ProjectivePoint!();
        AffinePoint!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl < C > GroupEncoding for ProjectivePoint < C > where C : PrimeCurveParams , CompressedPoint < C > : Send + Sync , FieldBytes < C > : Copy , FieldBytesSize < C > : ModulusSize , CompressedPoint < C > : Copy , < UncompressedPointSize < C > as ArraySize > :: ArrayType < u8 > : Copy , { type Repr = CompressedPoint < C > ; fn from_bytes (bytes : & Self :: Repr) -> CtOption < Self > { < AffinePoint < C > as GroupEncoding > :: from_bytes (bytes) . map (Into :: into) } fn from_bytes_unchecked (bytes : & Self :: Repr) -> CtOption < Self > { Self :: from_bytes (bytes) } fn to_bytes (& self) -> Self :: Repr { self . to_affine () . to_bytes () } }
    };
}

impl_69!();