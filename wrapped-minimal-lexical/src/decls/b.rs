macro_rules! deps {
    () => {
        ExtendedFloat!();
        Float!();
    };
}

macro_rules! b {
    () => {
        deps!();
        # [doc = " Calculate `b` from a a representation of `b` as a float."] # [inline] pub fn b < F : Float > (float : F) -> ExtendedFloat { ExtendedFloat { mant : float . mantissa () , exp : float . exponent () , } }
    };
}

b!()