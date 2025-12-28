macro_rules! deps {
    () => {
        PrimeCurveParams!();
        ProjectivePoint!();
        AffinePoint!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl < C > From < AffinePoint < C > > for ProjectivePoint < C > where C : PrimeCurveParams , { fn from (p : AffinePoint < C >) -> Self { let projective = ProjectivePoint { x : p . x , y : p . y , z : C :: FieldElement :: ONE , } ; Self :: conditional_select (& projective , & Self :: IDENTITY , p . is_identity ()) } }
    };
}

impl_60!()