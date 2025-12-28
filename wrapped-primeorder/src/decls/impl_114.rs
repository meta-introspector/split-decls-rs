macro_rules! deps {
    () => {
        AffinePoint!();
        PrimeCurveParams!();
        ProjectivePoint!();
    };
}

macro_rules! impl_114 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < 'de , C > Deserialize < 'de > for ProjectivePoint < C > where C : PrimeCurveParams , FieldBytes < C > : Copy , FieldBytesSize < C > : ModulusSize , CompressedPoint < C > : Copy , { fn deserialize < D > (deserializer : D) -> core :: result :: Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { AffinePoint :: < C > :: deserialize (deserializer) . map (Self :: from) } }
    };
}

impl_114!();