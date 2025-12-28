macro_rules! ceilf16 {
    () => {
        # [doc = " Ceil (f16)"] # [doc = ""] # [doc = " Finds the nearest integer greater than or equal to `x`."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn ceilf16 (x : f16) -> f16 { super :: generic :: ceil (x) }
    };
}

ceilf16!();