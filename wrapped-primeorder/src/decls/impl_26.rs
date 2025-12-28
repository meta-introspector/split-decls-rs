macro_rules! deps {
    () => {
        PrimeCurveParams!();
        AffinePoint!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl < C > FromEncodedPoint < C > for AffinePoint < C > where C : PrimeCurveParams , FieldBytes < C > : Copy , FieldBytesSize < C > : ModulusSize , CompressedPoint < C > : Copy , { # [doc = " Attempts to parse the given [`EncodedPoint`] as an SEC1-encoded"] # [doc = " [`AffinePoint`]."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " `None` value if `encoded_point` is not on the secp384r1 curve."] fn from_encoded_point (encoded_point : & EncodedPoint < C >) -> CtOption < Self > { match encoded_point . coordinates () { sec1 :: Coordinates :: Identity => CtOption :: new (Self :: IDENTITY , 1 . into ()) , sec1 :: Coordinates :: Compact { x } => Self :: decompact (x) , sec1 :: Coordinates :: Compressed { x , y_is_odd } => { Self :: decompress (x , Choice :: from (y_is_odd as u8)) } sec1 :: Coordinates :: Uncompressed { x , y } => Self :: from_coordinates (x , y) , } } }
    };
}

impl_26!();