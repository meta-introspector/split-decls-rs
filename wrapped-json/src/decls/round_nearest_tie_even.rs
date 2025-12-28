macro_rules! deps {
    () => {
        ExtendedFloat!();
    };
}

macro_rules! round_nearest_tie_even {
    () => {
        deps!();
        # [inline] pub (crate) fn round_nearest_tie_even (fp : & mut ExtendedFloat , shift : i32) { let (is_above , is_halfway) = round_nearest (fp , shift) ; tie_even (fp , is_above , is_halfway) ; }
    };
}

round_nearest_tie_even!()