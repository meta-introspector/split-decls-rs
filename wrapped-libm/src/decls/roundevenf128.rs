macro_rules! deps {
    () => {
        Round!();
    };
}

macro_rules! roundevenf128 {
    () => {
        deps!();
        # [doc = " Round `x` to the nearest integer, breaking ties toward even. This is IEEE 754"] # [doc = " `roundToIntegralTiesToEven`."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn roundevenf128 (x : f128) -> f128 { roundeven_impl (x) }
    };
}

roundevenf128!()