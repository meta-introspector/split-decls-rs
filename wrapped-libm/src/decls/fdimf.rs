macro_rules! fdimf {
    () => {
        # [doc = " Positive difference (f32)"] # [doc = ""] # [doc = " Determines the positive difference between arguments, returning:"] # [doc = " * x - y if x > y, or"] # [doc = " * +0    if x <= y, or"] # [doc = " * NAN   if either argument is NAN."] # [doc = ""] # [doc = " A range error may occur."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fdimf (x : f32 , y : f32) -> f32 { super :: generic :: fdim (x , y) }
    };
}

fdimf!();