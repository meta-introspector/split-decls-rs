macro_rules! sqrtf128 {
    () => {
        # [doc = " The square root of `x` (f128)."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn sqrtf128 (x : f128) -> f128 { return super :: generic :: sqrt (x) ; }
    };
}

sqrtf128!();