macro_rules! fminimum_num {
    () => {
        # [doc = " Return the lesser of two arguments or, if either argument is NaN, NaN."] # [doc = ""] # [doc = " This coincides with IEEE 754-2019 `minimumNumber`. The result orders -0.0 < 0.0."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fminimum_num (x : f64 , y : f64) -> f64 { super :: generic :: fminimum_num (x , y) }
    };
}

fminimum_num!()