macro_rules! fabsf16 {
    () => {
        # [doc = " Absolute value (magnitude) (f16)"] # [doc = ""] # [doc = " Calculates the absolute value (magnitude) of the argument `x`,"] # [doc = " by direct manipulation of the bit representation of `x`."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fabsf16 (x : f16) -> f16 { super :: generic :: fabs (x) }
    };
}

fabsf16!()