macro_rules! deps {
    () => {
        Float!();
        ExtendedFloat!();
    };
}

macro_rules! FloatErrors {
    () => {
        deps!();
        pub (crate) trait FloatErrors { # [doc = " Get the full error scale."] fn error_scale () -> u32 ; # [doc = " Get the half error scale."] fn error_halfscale () -> u32 ; # [doc = " Determine if the number of errors is tolerable for float precision."] fn error_is_accurate < F : Float > (count : u32 , fp : & ExtendedFloat) -> bool ; }
    };
}

FloatErrors!()