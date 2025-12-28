macro_rules! floorf128 {
    () => {
        # [doc = " Floor (f128)"] # [doc = ""] # [doc = " Finds the nearest integer less than or equal to `x`."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn floorf128 (x : f128) -> f128 { return super :: generic :: floor (x) ; }
    };
}

floorf128!();