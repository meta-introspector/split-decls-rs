macro_rules! fminimumf16 {
    () => {
        # [doc = " Return the lesser of two arguments or, if either argument is NaN, the other argument."] # [doc = ""] # [doc = " This coincides with IEEE 754-2019 `minimum`. The result orders -0.0 < 0.0."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fminimumf16 (x : f16 , y : f16) -> f16 { super :: generic :: fminimum (x , y) }
    };
}

fminimumf16!();