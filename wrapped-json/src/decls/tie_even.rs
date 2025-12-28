macro_rules! deps {
    () => {
        ExtendedFloat!();
    };
}

macro_rules! tie_even {
    () => {
        deps!();
        # [inline] pub (crate) fn tie_even (fp : & mut ExtendedFloat , is_above : bool , is_halfway : bool) { let is_odd = fp . mant & 1 == 1 ; if is_above || (is_odd && is_halfway) { fp . mant += 1 ; } }
    };
}

tie_even!()