macro_rules! deps {
    () => {
        Float!();
        ExtendedFloat!();
    };
}

macro_rules! bh_extended {
    () => {
        deps!();
        # [doc = " Calculate `b+h` from a representation of `b` as a float."] # [inline] pub (super) fn bh_extended < F : Float > (f : F) -> ExtendedFloat { let b = b_extended (f) ; ExtendedFloat { mant : (b . mant << 1) + 1 , exp : b . exp - 1 , } }
    };
}

bh_extended!()