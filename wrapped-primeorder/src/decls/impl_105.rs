macro_rules! deps {
    () => {
        ProjectivePoint!();
        AffinePoint!();
        PrimeCurveParams!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl < C > SubAssign < AffinePoint < C > > for ProjectivePoint < C > where C : PrimeCurveParams , { fn sub_assign (& mut self , rhs : AffinePoint < C >) { * self = ProjectivePoint :: sub_mixed (self , & rhs) ; } }
    };
}

impl_105!();