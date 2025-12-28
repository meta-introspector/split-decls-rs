macro_rules! deps {
    () => {
        PrimeCurveParams!();
        AffinePoint!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < C > AffinePoint < C > where C : PrimeCurveParams , { # [doc = " Additive identity of the group a.k.a. the point at infinity."] pub const IDENTITY : Self = Self { x : C :: FieldElement :: ZERO , y : C :: FieldElement :: ZERO , infinity : 1 , } ; # [doc = " Base point of the curve."] pub const GENERATOR : Self = Self { x : C :: GENERATOR . 0 , y : C :: GENERATOR . 1 , infinity : 0 , } ; # [doc = " Is this point the point at infinity?"] pub fn is_identity (& self) -> Choice { Choice :: from (self . infinity) } # [doc = " Conditionally negate [`AffinePoint`] for use with point compaction."] fn to_compact (self) -> Self { let neg_self = - self ; let choice = C :: Uint :: decode_field_bytes (& self . y . to_repr ()) . ct_gt (& C :: Uint :: decode_field_bytes (& neg_self . y . to_repr ())) ; Self { x : self . x , y : C :: FieldElement :: conditional_select (& self . y , & neg_self . y , choice) , infinity : self . infinity , } } }
    };
}

impl_16!();