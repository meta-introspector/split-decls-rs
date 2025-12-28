macro_rules! deps {
    () => {
        PrimeCurveParams!();
        ProjectivePoint!();
    };
}

macro_rules! impl_88 {
    () => {
        deps!();
        impl < C > AddAssign < ProjectivePoint < C > > for ProjectivePoint < C > where C : PrimeCurveParams , { fn add_assign (& mut self , rhs : ProjectivePoint < C >) { * self = ProjectivePoint :: add (self , & rhs) ; } }
    };
}

impl_88!();