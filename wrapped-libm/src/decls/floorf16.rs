macro_rules! floorf16 {
    () => {
        # [doc = " Floor (f16)"] # [doc = ""] # [doc = " Finds the nearest integer less than or equal to `x`."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn floorf16 (x : f16) -> f16 { return super :: generic :: floor (x) ; }
    };
}

floorf16!()