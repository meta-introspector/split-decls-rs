
macro_rules! fmodf16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmodf16 in module {}", module_path!());
    };
}

mkfn!{
    fmodf16_introspect!();
    # [doc = " Calculate the remainder of `x / y`, the precise result of `x - trunc(x / y) * y`."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmodf16 (x : f16 , y : f16) -> f16 { super :: generic :: fmod (x , y) }
}

macro_rules! fmodf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmodf in module {}", module_path!());
    };
}

mkfn!{
    fmodf_introspect!();
    # [doc = " Calculate the remainder of `x / y`, the precise result of `x - trunc(x / y) * y`."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmodf (x : f32 , y : f32) -> f32 { super :: generic :: fmod (x , y) }
}

macro_rules! fmod_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmod in module {}", module_path!());
    };
}

mkfn!{
    fmod_introspect!();
    # [doc = " Calculate the remainder of `x / y`, the precise result of `x - trunc(x / y) * y`."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmod (x : f64 , y : f64) -> f64 { super :: generic :: fmod (x , y) }
}

macro_rules! fmodf128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmodf128 in module {}", module_path!());
    };
}

mkfn!{
    fmodf128_introspect!();
    # [doc = " Calculate the remainder of `x / y`, the precise result of `x - trunc(x / y) * y`."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn fmodf128 (x : f128 , y : f128) -> f128 { super :: generic :: fmod (x , y) }
}