macro_rules! deps {
    () => {
        NistP384!();
    };
}

macro_rules! NonZeroScalar {
    () => {
        deps!();
        # [doc = " Non-zero NIST P-384 scalar field element."] # [cfg (feature = "arithmetic")] pub type NonZeroScalar = elliptic_curve :: NonZeroScalar < NistP384 > ;
    };
}

NonZeroScalar!();