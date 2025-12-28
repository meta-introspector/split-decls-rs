macro_rules! deps {
    () => {
        Float!();
        ExtendedFloat!();
    };
}

macro_rules! moderate_path {
    () => {
        deps!();
        # [doc = " Create a precise native float using an intermediate extended-precision float."] # [doc = ""] # [doc = " Return the float approximation and if the value can be accurately"] # [doc = " represented with mantissa bits of precision."] # [inline] pub (crate) fn moderate_path < F > (mantissa : u64 , exponent : i32 , truncated : bool ,) -> (ExtendedFloat , bool) where F : Float , { let mut fp = ExtendedFloat { mant : mantissa , exp : 0 , } ; let valid = multiply_exponent_extended :: < F > (& mut fp , exponent , truncated) ; (fp , valid) }
    };
}

moderate_path!()