
macro_rules! sqrtf16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sqrtf16 in module {}", module_path!());
    };
}

mkfn!{
    sqrtf16_introspect!();
    # [doc = " The square root of `x` (f16)."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn sqrtf16 (x : f16) -> f16 { select_implementation ! { name : sqrtf16 , use_arch : all (target_arch = "aarch64" , target_feature = "fp16") , args : x , } return super :: generic :: sqrt (x) ; }
}

macro_rules! sqrtf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sqrtf in module {}", module_path!());
    };
}

mkfn!{
    sqrtf_introspect!();
    # [doc = " The square root of `x` (f32)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn sqrtf (x : f32) -> f32 { select_implementation ! { name : sqrtf , use_arch : any (all (target_arch = "aarch64" , target_feature = "neon") , all (target_arch = "wasm32" , intrinsics_enabled) , target_feature = "sse2") , args : x , } super :: generic :: sqrt (x) }
}

macro_rules! sqrt_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sqrt in module {}", module_path!());
    };
}

mkfn!{
    sqrt_introspect!();
    # [doc = " The square root of `x` (f64)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn sqrt (x : f64) -> f64 { select_implementation ! { name : sqrt , use_arch : any (all (target_arch = "aarch64" , target_feature = "neon") , all (target_arch = "wasm32" , intrinsics_enabled) , target_feature = "sse2") , args : x , } super :: generic :: sqrt (x) }
}

macro_rules! sqrtf128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sqrtf128 in module {}", module_path!());
    };
}

mkfn!{
    sqrtf128_introspect!();
    # [doc = " The square root of `x` (f128)."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn sqrtf128 (x : f128) -> f128 { return super :: generic :: sqrt (x) ; }
}