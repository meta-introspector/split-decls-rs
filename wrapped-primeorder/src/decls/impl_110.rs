macro_rules! deps {
    () => {
        ProjectivePoint!();
        PrimeCurveParams!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl < C , S > MulAssign < S > for ProjectivePoint < C > where Self : Double , C : PrimeCurveParams , S : Borrow < Scalar < C > > , { fn mul_assign (& mut self , scalar : S) { * self = ProjectivePoint :: mul (self , scalar . borrow ()) ; } }
    };
}

impl_110!();