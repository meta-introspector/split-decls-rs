macro_rules! deps {
    () => {
        LenUint!();
    };
}

macro_rules! assert_capacity_limit {
    () => {
        deps!();
        macro_rules ! assert_capacity_limit { ($ cap : expr) => { if std :: mem :: size_of ::< usize > () > std :: mem :: size_of ::< LenUint > () { if $ cap > LenUint :: MAX as usize { # [cfg (not (target_pointer_width = "16"))] panic ! ("ArrayVec: largest supported capacity is u32::MAX") ; # [cfg (target_pointer_width = "16")] panic ! ("ArrayVec: largest supported capacity is u16::MAX") ; } } } ; }
    };
}

assert_capacity_limit!()