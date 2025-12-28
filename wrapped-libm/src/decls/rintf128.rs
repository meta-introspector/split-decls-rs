macro_rules! deps {
    () => {
        Round!();
    };
}

macro_rules! rintf128 {
    () => {
        deps!();
        # [doc = " Round `x` to the nearest integer, breaking ties toward even."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn rintf128 (x : f128) -> f128 { super :: generic :: rint_round (x , Round :: Nearest) . val }
    };
}

rintf128!()