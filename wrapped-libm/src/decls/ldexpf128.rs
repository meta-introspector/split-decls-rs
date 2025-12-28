macro_rules! ldexpf128 {
    () => {
        # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn ldexpf128 (x : f128 , n : i32) -> f128 { super :: scalbnf128 (x , n) }
    };
}

ldexpf128!();