macro_rules! deps {
    () => {
        PrimeCurveParams!();
        AffinePoint!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        impl < C > TryFrom < & EncodedPoint < C > > for AffinePoint < C > where C : PrimeCurveParams , FieldBytes < C > : Copy , FieldBytesSize < C > : ModulusSize , CompressedPoint < C > : Copy , { type Error = Error ; fn try_from (point : & EncodedPoint < C >) -> Result < AffinePoint < C > > { Option :: from (AffinePoint :: < C > :: from_encoded_point (point)) . ok_or (Error) } }
    };
}

impl_40!();