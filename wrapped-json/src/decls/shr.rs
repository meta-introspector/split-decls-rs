macro_rules! deps {
    () => {
        ExtendedFloat!();
    };
}

macro_rules! shr {
    () => {
        deps!();
        # [inline] pub (crate) fn shr (fp : & mut ExtendedFloat , shift : i32) { let bits : u64 = mem :: size_of :: < u64 > () as u64 * 8 ; debug_assert ! ((shift as u64) < bits , "shr() overflow in shift right.") ; fp . mant >>= shift ; fp . exp += shift ; }
    };
}

shr!();