macro_rules! deps {
    () => {
        ExtendedFloat!();
        Float!();
    };
}

macro_rules! b_extended {
    () => {
        deps!();
        # [doc = " Calculate `b` from a representation of `b` as a float."] # [inline] pub (super) fn b_extended < F : Float > (f : F) -> ExtendedFloat { ExtendedFloat :: from_float (f) }
    };
}

b_extended!()