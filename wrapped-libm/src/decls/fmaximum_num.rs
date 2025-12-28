macro_rules! fmaximum_num {
    () => {
        # [doc = " Return the greater of two arguments or, if either argument is NaN, NaN."] # [doc = ""] # [doc = " This coincides with IEEE 754-2019 `maximumNumber`. The result orders -0.0 < 0.0."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmaximum_num (x : f64 , y : f64) -> f64 { super :: generic :: fmaximum_num (x , y) }
    };
}

fmaximum_num!();