macro_rules! deps {
    () => {
        AffinePoint!();
        OsswuMap!();
        PrimeCurveParams!();
    };
}

macro_rules! AffineOsswuMap {
    () => {
        deps!();
        # [doc = " [`OsswuMap`] for [`AffinePoint`]."] pub trait AffineOsswuMap < C : PrimeCurveParams < FieldElement : OsswuMap > > { # [doc = " [`OsswuMap::osswu()`] to [`AffinePoint`]."] fn osswu (u : & C :: FieldElement) -> Self ; }
    };
}

AffineOsswuMap!()