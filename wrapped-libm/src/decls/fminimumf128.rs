macro_rules! fminimumf128 {
    () => {
        # [doc = " Return the lesser of two arguments or, if either argument is NaN, the other argument."] # [doc = ""] # [doc = " This coincides with IEEE 754-2019 `minimum`. The result orders -0.0 < 0.0."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fminimumf128 (x : f128 , y : f128) -> f128 { super :: generic :: fminimum (x , y) }
    };
}

fminimumf128!();