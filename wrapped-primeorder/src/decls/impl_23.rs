macro_rules! deps {
    () => {
        AffinePoint!();
        PrimeCurveParams!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < C > DecompressPoint < C > for AffinePoint < C > where C : PrimeCurveParams , FieldBytes < C > : Copy , { fn decompress (x_bytes : & FieldBytes < C > , y_is_odd : Choice) -> CtOption < Self > { C :: FieldElement :: from_repr (* x_bytes) . and_then (| x | { let alpha = x * & x * & x + & (C :: EQUATION_A * & x) + & C :: EQUATION_B ; let beta = alpha . sqrt () ; beta . map (| beta | { let y = C :: FieldElement :: conditional_select (& - beta , & beta , beta . is_odd () . ct_eq (& y_is_odd) ,) ; Self { x , y , infinity : 0 } }) }) } }
    };
}

impl_23!();