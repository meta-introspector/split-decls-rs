macro_rules! deps {
    () => {
        Round!();
    };
}

macro_rules! roundevenf {
    () => {
        deps!();
        # [doc = " Round `x` to the nearest integer, breaking ties toward even. This is IEEE 754"] # [doc = " `roundToIntegralTiesToEven`."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn roundevenf (x : f32) -> f32 { roundeven_impl (x) }
    };
}

roundevenf!()