macro_rules! deps {
    () => {
        PrimeCurveParams!();
        OsswuMap!();
        AffineOsswuMap!();
        AffinePoint!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl < C : PrimeCurveParams < FieldElement : OsswuMap > > AffineOsswuMap < C > for AffinePoint < C > { fn osswu (u : & < C as PrimeCurveParams > :: FieldElement) -> Self { let (x , y) = u . osswu () ; Self { x , y , infinity : 0 } } }
    };
}

impl_2!();