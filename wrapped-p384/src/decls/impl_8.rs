macro_rules! deps {
    () => {
        NistP384!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl elliptic_curve :: point :: PointCompression for NistP384 { # [doc = " NIST P-384 points are typically uncompressed."] const COMPRESS_POINTS : bool = false ; }
    };
}

impl_8!()