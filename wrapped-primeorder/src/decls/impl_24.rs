macro_rules! deps {
    () => {
        AffinePoint!();
        PrimeCurveParams!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl < C > DecompactPoint < C > for AffinePoint < C > where C : PrimeCurveParams , FieldBytes < C > : Copy , { fn decompact (x_bytes : & FieldBytes < C >) -> CtOption < Self > { Self :: decompress (x_bytes , Choice :: from (0)) . map (| point | point . to_compact ()) } }
    };
}

impl_24!();