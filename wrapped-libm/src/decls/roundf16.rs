macro_rules! deps {
    () => {
        Round!();
    };
}

macro_rules! roundf16 {
    () => {
        deps!();
        # [doc = " Round `x` to the nearest integer, breaking ties away from zero."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn roundf16 (x : f16) -> f16 { super :: generic :: round (x) }
    };
}

roundf16!()