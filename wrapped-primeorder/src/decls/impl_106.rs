macro_rules! deps {
    () => {
        ProjectivePoint!();
        PrimeCurveParams!();
        AffinePoint!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl < C > SubAssign < & AffinePoint < C > > for ProjectivePoint < C > where C : PrimeCurveParams , { fn sub_assign (& mut self , rhs : & AffinePoint < C >) { * self = ProjectivePoint :: sub_mixed (self , rhs) ; } }
    };
}

impl_106!()