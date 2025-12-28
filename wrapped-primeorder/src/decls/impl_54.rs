macro_rules! deps {
    () => {
        ProjectivePoint!();
        PrimeCurveParams!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < C > ConditionallySelectable for ProjectivePoint < C > where C : PrimeCurveParams , { # [inline (always)] fn conditional_select (a : & Self , b : & Self , choice : Choice) -> Self { Self { x : C :: FieldElement :: conditional_select (& a . x , & b . x , choice) , y : C :: FieldElement :: conditional_select (& a . y , & b . y , choice) , z : C :: FieldElement :: conditional_select (& a . z , & b . z , choice) , } } }
    };
}

impl_54!()