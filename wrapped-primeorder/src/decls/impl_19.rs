macro_rules! deps {
    () => {
        PrimeCurveParams!();
        AffinePoint!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < C > ConditionallySelectable for AffinePoint < C > where C : PrimeCurveParams , { # [inline (always)] fn conditional_select (a : & Self , b : & Self , choice : Choice) -> Self { Self { x : C :: FieldElement :: conditional_select (& a . x , & b . x , choice) , y : C :: FieldElement :: conditional_select (& a . y , & b . y , choice) , infinity : u8 :: conditional_select (& a . infinity , & b . infinity , choice) , } } }
    };
}

impl_19!();