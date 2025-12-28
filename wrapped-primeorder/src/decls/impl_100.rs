macro_rules! deps {
    () => {
        ProjectivePoint!();
        PrimeCurveParams!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl < C > SubAssign < ProjectivePoint < C > > for ProjectivePoint < C > where C : PrimeCurveParams , { fn sub_assign (& mut self , rhs : ProjectivePoint < C >) { * self = ProjectivePoint :: sub (self , & rhs) ; } }
    };
}

impl_100!()