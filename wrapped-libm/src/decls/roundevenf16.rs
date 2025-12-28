macro_rules! deps {
    () => {
        Round!();
    };
}

macro_rules! roundevenf16 {
    () => {
        deps!();
        # [doc = " Round `x` to the nearest integer, breaking ties toward even. This is IEEE 754"] # [doc = " `roundToIntegralTiesToEven`."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn roundevenf16 (x : f16) -> f16 { roundeven_impl (x) }
    };
}

roundevenf16!();