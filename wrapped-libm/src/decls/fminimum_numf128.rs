macro_rules! fminimum_numf128 {
    () => {
        # [doc = " Return the lesser of two arguments or, if either argument is NaN, NaN."] # [doc = ""] # [doc = " This coincides with IEEE 754-2019 `minimumNumber`. The result orders -0.0 < 0.0."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fminimum_numf128 (x : f128 , y : f128) -> f128 { super :: generic :: fminimum_num (x , y) }
    };
}

fminimum_numf128!();