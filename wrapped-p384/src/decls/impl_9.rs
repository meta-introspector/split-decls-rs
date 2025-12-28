macro_rules! deps {
    () => {
        NistP384!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl elliptic_curve :: point :: PointCompaction for NistP384 { # [doc = " NIST P-384 points are typically uncompressed."] const COMPACT_POINTS : bool = false ; }
    };
}

impl_9!()