macro_rules! fdimf16 {
    () => {
        # [doc = " Positive difference (f16)"] # [doc = ""] # [doc = " Determines the positive difference between arguments, returning:"] # [doc = " * x - y if x > y, or"] # [doc = " * +0    if x <= y, or"] # [doc = " * NAN   if either argument is NAN."] # [doc = ""] # [doc = " A range error may occur."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fdimf16 (x : f16 , y : f16) -> f16 { super :: generic :: fdim (x , y) }
    };
}

fdimf16!()