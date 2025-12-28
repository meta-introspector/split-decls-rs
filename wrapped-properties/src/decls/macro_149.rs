macro_rules! deps {
    () => {
        VerticalOrientation!();
    };
}

macro_rules! macro_149 {
    () => {
        deps!();
        create_const_array ! { # [allow (missing_docs)] # [allow (non_upper_case_globals)] impl VerticalOrientation { pub const Rotated : VerticalOrientation = VerticalOrientation (0) ; pub const TransformedRotated : VerticalOrientation = VerticalOrientation (1) ; pub const TransformedUpright : VerticalOrientation = VerticalOrientation (2) ; pub const Upright : VerticalOrientation = VerticalOrientation (3) ; } }
    };
}

macro_149!();