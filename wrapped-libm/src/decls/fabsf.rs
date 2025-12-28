macro_rules! fabsf {
    () => {
        # [doc = " Absolute value (magnitude) (f32)"] # [doc = ""] # [doc = " Calculates the absolute value (magnitude) of the argument `x`,"] # [doc = " by direct manipulation of the bit representation of `x`."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fabsf (x : f32) -> f32 { select_implementation ! { name : fabsf , use_arch : all (target_arch = "wasm32" , intrinsics_enabled) , args : x , } super :: generic :: fabs (x) }
    };
}

fabsf!()