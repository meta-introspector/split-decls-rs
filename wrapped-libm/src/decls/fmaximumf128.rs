macro_rules! fmaximumf128 {
    () => {
        # [doc = " Return the greater of two arguments or, if either argument is NaN, the other argument."] # [doc = ""] # [doc = " This coincides with IEEE 754-2019 `maximum`. The result orders -0.0 < 0.0."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmaximumf128 (x : f128 , y : f128) -> f128 { super :: generic :: fmaximum (x , y) }
    };
}

fmaximumf128!();