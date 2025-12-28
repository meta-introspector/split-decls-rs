macro_rules! deps {
    () => {
        Fallback!();
        DoubleFloat!();
        FloatConvert!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl < F : FloatConvert < Self > > From < DoubleFloat < F > > for Fallback < F > { fn from (DoubleFloat (a , b) : DoubleFloat < F >) -> Self { let mut status ; let mut loses_info = false ; let a = unpack ! (status =, a . convert (& mut loses_info)) ; assert_eq ! ((status , loses_info) , (Status :: OK , false)) ; if a . is_finite_non_zero () { let b = unpack ! (status =, b . convert (& mut loses_info)) ; assert_eq ! ((status , loses_info) , (Status :: OK , false)) ; (a + b) . value } else { a } } }
    };
}

impl_60!()