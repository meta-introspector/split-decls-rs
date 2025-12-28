macro_rules! deps {
    () => {
        ExtendedFloat!();
    };
}

macro_rules! shl {
    () => {
        deps!();
        # [inline] pub (crate) fn shl (fp : & mut ExtendedFloat , shift : i32) { let bits : u64 = mem :: size_of :: < u64 > () as u64 * 8 ; debug_assert ! ((shift as u64) < bits , "shl() overflow in shift left.") ; fp . mant <<= shift ; fp . exp -= shift ; }
    };
}

shl!();