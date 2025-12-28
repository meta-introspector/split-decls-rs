macro_rules! deps {
    () => {
        PrimeCurveParams!();
        ProjectivePoint!();
        AffinePoint!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < C > CurveGroup for ProjectivePoint < C > where C : PrimeCurveParams , FieldBytes < C > : Copy , { type AffineRepr = AffinePoint < C > ; fn to_affine (& self) -> AffinePoint < C > { ProjectivePoint :: to_affine (self) } # [cfg (feature = "alloc")] # [inline] fn batch_normalize (projective : & [Self] , affine : & mut [Self :: AffineRepr]) { assert_eq ! (projective . len () , affine . len ()) ; let mut zs = vec ! [C :: FieldElement :: ONE ; projective . len ()] ; batch_normalize_generic (projective , zs . as_mut_slice () , affine) ; } }
    };
}

impl_67!()