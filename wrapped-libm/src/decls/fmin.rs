macro_rules! fmin {
    () => {
        # [doc = " Return the lesser of two arguments or, if either argument is NaN, the other argument."] # [doc = ""] # [doc = " This coincides with IEEE 754-2011 `minNum`. The result disregards signed zero (meaning if"] # [doc = " the inputs are -0.0 and +0.0, either may be returned)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmin (x : f64 , y : f64) -> f64 { super :: generic :: fmin (x , y) }
    };
}

fmin!();