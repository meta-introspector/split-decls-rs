macro_rules! deps {
    () => {
        PrimeCurveParams!();
        AffinePoint!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < 'de , C > Deserialize < 'de > for AffinePoint < C > where C : PrimeCurveParams , FieldBytes < C > : Copy , FieldBytesSize < C > : ModulusSize , CompressedPoint < C > : Copy , { fn deserialize < D > (deserializer : D) -> core :: result :: Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { EncodedPoint :: < C > :: deserialize (deserializer) ? . try_into () . map_err (de :: Error :: custom) } }
    };
}

impl_48!()