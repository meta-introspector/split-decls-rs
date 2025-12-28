macro_rules! truncf {
    () => {
        # [doc = " Rounds the number toward 0 to the closest integral value (f32)."] # [doc = ""] # [doc = " This effectively removes the decimal part of the number, leaving the integral part."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn truncf (x : f32) -> f32 { select_implementation ! { name : truncf , use_arch : all (target_arch = "wasm32" , intrinsics_enabled) , args : x , } super :: generic :: trunc (x) }
    };
}

truncf!()