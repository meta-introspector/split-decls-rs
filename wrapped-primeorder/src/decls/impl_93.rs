macro_rules! deps {
    () => {
        ProjectivePoint!();
        PrimeCurveParams!();
        AffinePoint!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl < C > AddAssign < AffinePoint < C > > for ProjectivePoint < C > where C : PrimeCurveParams , { fn add_assign (& mut self , rhs : AffinePoint < C >) { * self = ProjectivePoint :: add_mixed (self , & rhs) ; } }
    };
}

impl_93!();