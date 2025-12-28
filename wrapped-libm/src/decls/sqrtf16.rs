macro_rules! sqrtf16 {
    () => {
        # [doc = " The square root of `x` (f16)."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn sqrtf16 (x : f16) -> f16 { select_implementation ! { name : sqrtf16 , use_arch : all (target_arch = "aarch64" , target_feature = "fp16") , args : x , } return super :: generic :: sqrt (x) ; }
    };
}

sqrtf16!()