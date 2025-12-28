macro_rules! CompressedPoint {
    () => {
        # [doc = " Compressed SEC1-encoded NIST P-384 curve point."] pub type CompressedPoint = Array < u8 , U49 > ;
    };
}

CompressedPoint!();