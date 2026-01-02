
macro_rules! ceilf16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ceilf16 in module {}", module_path!());
    };
}

mkfn!{
    ceilf16_introspect!();
    # [doc = " Ceil (f16)"] # [doc = ""] # [doc = " Finds the nearest integer greater than or equal to `x`."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn ceilf16 (x : f16) -> f16 { super :: generic :: ceil (x) }
}

macro_rules! ceilf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ceilf in module {}", module_path!());
    };
}

mkfn!{
    ceilf_introspect!();
    # [doc = " Ceil (f32)"] # [doc = ""] # [doc = " Finds the nearest integer greater than or equal to `x`."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn ceilf (x : f32) -> f32 { select_implementation ! { name : ceilf , use_arch : all (target_arch = "wasm32" , intrinsics_enabled) , args : x , } super :: generic :: ceil (x) }
}

macro_rules! ceil_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ceil in module {}", module_path!());
    };
}

mkfn!{
    ceil_introspect!();
    # [doc = " Ceil (f64)"] # [doc = ""] # [doc = " Finds the nearest integer greater than or equal to `x`."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn ceil (x : f64) -> f64 { select_implementation ! { name : ceil , use_arch : all (target_arch = "wasm32" , intrinsics_enabled) , use_arch_required : all (target_arch = "x86" , not (target_feature = "sse2")) , args : x , } super :: generic :: ceil (x) }
}

macro_rules! ceilf128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ceilf128 in module {}", module_path!());
    };
}

mkfn!{
    ceilf128_introspect!();
    # [doc = " Ceil (f128)"] # [doc = ""] # [doc = " Finds the nearest integer greater than or equal to `x`."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn ceilf128 (x : f128) -> f128 { super :: generic :: ceil (x) }
}