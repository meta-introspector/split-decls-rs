macro_rules! fdimf128 {
    () => {
        # [doc = " Positive difference (f128)"] # [doc = ""] # [doc = " Determines the positive difference between arguments, returning:"] # [doc = " * x - y if x > y, or"] # [doc = " * +0    if x <= y, or"] # [doc = " * NAN   if either argument is NAN."] # [doc = ""] # [doc = " A range error may occur."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fdimf128 (x : f128 , y : f128) -> f128 { super :: generic :: fdim (x , y) }
    };
}

fdimf128!();