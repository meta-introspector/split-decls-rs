macro_rules! deps {
    () => {
        PrimeCurveParams!();
        AffinePoint!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < C > AffineCoordinates for AffinePoint < C > where C : PrimeCurveParams , FieldBytes < C > : Copy , { type FieldRepr = FieldBytes < C > ; fn from_coordinates (x : & Self :: FieldRepr , y : & Self :: FieldRepr) -> CtOption < Self > { C :: FieldElement :: from_repr (* y) . and_then (| y | { C :: FieldElement :: from_repr (* x) . and_then (| x | { let lhs = y * & y ; let rhs = x * & x * & x + & (C :: EQUATION_A * & x) + & C :: EQUATION_B ; CtOption :: new (Self { x , y , infinity : 0 } , lhs . ct_eq (& rhs)) }) }) } fn x (& self) -> FieldBytes < C > { self . x . to_repr () } fn y (& self) -> FieldBytes < C > { self . y . to_repr () } fn x_is_odd (& self) -> Choice { self . x . is_odd () } fn y_is_odd (& self) -> Choice { self . y . is_odd () } }
    };
}

impl_18!();