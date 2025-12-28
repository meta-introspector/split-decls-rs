macro_rules! fabsf128 {
    () => {
        # [doc = " Absolute value (magnitude) (f128)"] # [doc = ""] # [doc = " Calculates the absolute value (magnitude) of the argument `x`,"] # [doc = " by direct manipulation of the bit representation of `x`."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fabsf128 (x : f128) -> f128 { super :: generic :: fabs (x) }
    };
}

fabsf128!();