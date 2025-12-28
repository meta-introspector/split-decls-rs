macro_rules! fminimum {
    () => {
        # [doc = " Return the lesser of two arguments or, if either argument is NaN, the other argument."] # [doc = ""] # [doc = " This coincides with IEEE 754-2019 `minimum`. The result orders -0.0 < 0.0."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fminimum (x : f64 , y : f64) -> f64 { super :: generic :: fminimum (x , y) }
    };
}

fminimum!()