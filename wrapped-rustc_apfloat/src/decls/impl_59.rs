macro_rules! deps {
    () => {
        Float!();
        FloatConvert!();
        DoubleFloat!();
        FallbackExtended!();
        Fallback!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl < F : Float > From < Fallback < F > > for DoubleFloat < F > where F : FloatConvert < FallbackExtended < F > > , FallbackExtended < F > : FloatConvert < F > , { fn from (x : Fallback < F >) -> Self { let mut status ; let mut loses_info = false ; let extended : FallbackExtended < F > = unpack ! (status =, x . convert (& mut loses_info)) ; assert_eq ! ((status , loses_info) , (Status :: OK , false)) ; let a = unpack ! (status =, extended . convert (& mut loses_info)) ; assert_eq ! (status - Status :: INEXACT , Status :: OK) ; let b = if a . is_finite_non_zero () && loses_info { let u : FallbackExtended < F > = unpack ! (status =, a . convert (& mut loses_info)) ; assert_eq ! ((status , loses_info) , (Status :: OK , false)) ; let v = unpack ! (status =, extended - u) ; assert_eq ! (status , Status :: OK) ; let v = unpack ! (status =, v . convert (& mut loses_info)) ; assert_eq ! ((status , loses_info) , (Status :: OK , false)) ; v } else { F :: ZERO } ; DoubleFloat (a , b) } }
    };
}

impl_59!()