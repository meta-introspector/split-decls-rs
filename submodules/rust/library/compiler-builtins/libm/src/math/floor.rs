
macro_rules! floorf16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function floorf16 in module {}", module_path!());
    };
}

mkfn!{
    floorf16_introspect!();
    # [doc = " Floor (f16)"] # [doc = ""] # [doc = " Finds the nearest integer less than or equal to `x`."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn floorf16 (x : f16) -> f16 { return super :: generic :: floor (x) ; }
}

macro_rules! floor_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function floor in module {}", module_path!());
    };
}

mkfn!{
    floor_introspect!();
    # [doc = " Floor (f64)"] # [doc = ""] # [doc = " Finds the nearest integer less than or equal to `x`."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn floor (x : f64) -> f64 { select_implementation ! { name : floor , use_arch : all (target_arch = "wasm32" , intrinsics_enabled) , use_arch_required : all (target_arch = "x86" , not (target_feature = "sse2")) , args : x , } return super :: generic :: floor (x) ; }
}

macro_rules! floorf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function floorf in module {}", module_path!());
    };
}

mkfn!{
    floorf_introspect!();
    # [doc = " Floor (f32)"] # [doc = ""] # [doc = " Finds the nearest integer less than or equal to `x`."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn floorf (x : f32) -> f32 { select_implementation ! { name : floorf , use_arch : all (target_arch = "wasm32" , intrinsics_enabled) , args : x , } return super :: generic :: floor (x) ; }
}

macro_rules! floorf128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function floorf128 in module {}", module_path!());
    };
}

mkfn!{
    floorf128_introspect!();
    # [doc = " Floor (f128)"] # [doc = ""] # [doc = " Finds the nearest integer less than or equal to `x`."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn floorf128 (x : f128) -> f128 { return super :: generic :: floor (x) ; }
}