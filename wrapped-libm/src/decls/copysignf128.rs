macro_rules! copysignf128 {
    () => {
        # [doc = " Sign of Y, magnitude of X (f128)"] # [doc = ""] # [doc = " Constructs a number with the magnitude (absolute value) of its"] # [doc = " first argument, `x`, and the sign of its second argument, `y`."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn copysignf128 (x : f128 , y : f128) -> f128 { super :: generic :: copysign (x , y) }
    };
}

copysignf128!();