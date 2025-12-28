macro_rules! deps {
    () => {
        AffinePoint!();
        PrimeCurveParams!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < C > Neg for AffinePoint < C > where C : PrimeCurveParams , { type Output = Self ; # [inline] fn neg (self) -> Self { AffinePoint { x : self . x , y : - self . y , infinity : self . infinity , } } }
    };
}

impl_45!()