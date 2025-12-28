macro_rules! deps {
    () => {
        AffinePoint!();
        PrimeCurveParams!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < C > Neg for & AffinePoint < C > where C : PrimeCurveParams , { type Output = AffinePoint < C > ; # [inline] fn neg (self) -> AffinePoint < C > { - (* self) } }
    };
}

impl_46!()