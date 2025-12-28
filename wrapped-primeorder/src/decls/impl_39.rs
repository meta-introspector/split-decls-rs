macro_rules! deps {
    () => {
        AffinePoint!();
        PrimeCurveParams!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < C > TryFrom < EncodedPoint < C > > for AffinePoint < C > where C : PrimeCurveParams , FieldBytes < C > : Copy , FieldBytesSize < C > : ModulusSize , CompressedPoint < C > : Copy , { type Error = Error ; fn try_from (point : EncodedPoint < C >) -> Result < AffinePoint < C > > { AffinePoint :: try_from (& point) } }
    };
}

impl_39!()