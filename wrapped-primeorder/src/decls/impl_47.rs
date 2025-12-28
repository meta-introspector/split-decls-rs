macro_rules! deps {
    () => {
        AffinePoint!();
        PrimeCurveParams!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        # [cfg (feature = "serde")] impl < C > Serialize for AffinePoint < C > where C : PrimeCurveParams , FieldBytesSize < C > : ModulusSize , CompressedPoint < C > : Copy , < UncompressedPointSize < C > as ArraySize > :: ArrayType < u8 > : Copy , { fn serialize < S > (& self , serializer : S) -> core :: result :: Result < S :: Ok , S :: Error > where S : ser :: Serializer , { self . to_encoded_point (true) . serialize (serializer) } }
    };
}

impl_47!()