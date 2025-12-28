macro_rules! ceilf128 {
    () => {
        # [doc = " Ceil (f128)"] # [doc = ""] # [doc = " Finds the nearest integer greater than or equal to `x`."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn ceilf128 (x : f128) -> f128 { super :: generic :: ceil (x) }
    };
}

ceilf128!()