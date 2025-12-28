macro_rules! fminimum_numf16 {
    () => {
        # [doc = " Return the lesser of two arguments or, if either argument is NaN, NaN."] # [doc = ""] # [doc = " This coincides with IEEE 754-2019 `minimumNumber`. The result orders -0.0 < 0.0."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fminimum_numf16 (x : f16 , y : f16) -> f16 { super :: generic :: fminimum_num (x , y) }
    };
}

fminimum_numf16!()