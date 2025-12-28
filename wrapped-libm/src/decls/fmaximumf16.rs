macro_rules! fmaximumf16 {
    () => {
        # [doc = " Return the greater of two arguments or, if either argument is NaN, the other argument."] # [doc = ""] # [doc = " This coincides with IEEE 754-2019 `maximum`. The result orders -0.0 < 0.0."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmaximumf16 (x : f16 , y : f16) -> f16 { super :: generic :: fmaximum (x , y) }
    };
}

fmaximumf16!();