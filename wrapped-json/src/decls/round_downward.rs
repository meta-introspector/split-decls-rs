macro_rules! deps {
    () => {
        ExtendedFloat!();
    };
}

macro_rules! round_downward {
    () => {
        deps!();
        # [inline] pub (crate) fn round_downward (fp : & mut ExtendedFloat , shift : i32) { let is_truncated = round_toward (fp , shift) ; downard (fp , is_truncated) ; }
    };
}

round_downward!();