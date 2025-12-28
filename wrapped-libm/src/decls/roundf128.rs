macro_rules! deps {
    () => {
        Round!();
    };
}

macro_rules! roundf128 {
    () => {
        deps!();
        # [doc = " Round `x` to the nearest integer, breaking ties away from zero."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn roundf128 (x : f128) -> f128 { super :: generic :: round (x) }
    };
}

roundf128!();