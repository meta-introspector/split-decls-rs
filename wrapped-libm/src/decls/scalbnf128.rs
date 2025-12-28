macro_rules! scalbnf128 {
    () => {
        # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn scalbnf128 (x : f128 , n : i32) -> f128 { super :: generic :: scalbn (x , n) }
    };
}

scalbnf128!();