macro_rules! fmaximumf {
    () => {
        # [doc = " Return the greater of two arguments or, if either argument is NaN, the other argument."] # [doc = ""] # [doc = " This coincides with IEEE 754-2019 `maximum`. The result orders -0.0 < 0.0."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmaximumf (x : f32 , y : f32) -> f32 { super :: generic :: fmaximum (x , y) }
    };
}

fmaximumf!()