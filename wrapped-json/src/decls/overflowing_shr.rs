macro_rules! deps {
    () => {
        ExtendedFloat!();
    };
}

macro_rules! overflowing_shr {
    () => {
        deps!();
        # [inline] pub (crate) fn overflowing_shr (fp : & mut ExtendedFloat , shift : i32) { let bits : u64 = mem :: size_of :: < u64 > () as u64 * 8 ; debug_assert ! ((shift as u64) <= bits , "overflowing_shr() overflow in shift right.") ; fp . mant = if shift as u64 == bits { 0 } else { fp . mant >> shift } ; fp . exp += shift ; }
    };
}

overflowing_shr!()