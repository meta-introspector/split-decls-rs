
macro_rules! roundf16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function roundf16 in module {}", module_path!());
    };
}

mkfn!{
    roundf16_introspect!();
    # [doc = " Round `x` to the nearest integer, breaking ties away from zero."] # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn roundf16 (x : f16) -> f16 { super :: generic :: round (x) }
}

macro_rules! roundf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function roundf in module {}", module_path!());
    };
}

mkfn!{
    roundf_introspect!();
    # [doc = " Round `x` to the nearest integer, breaking ties away from zero."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn roundf (x : f32) -> f32 { super :: generic :: round (x) }
}

macro_rules! round_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function round in module {}", module_path!());
    };
}

mkfn!{
    round_introspect!();
    # [doc = " Round `x` to the nearest integer, breaking ties away from zero."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn round (x : f64) -> f64 { super :: generic :: round (x) }
}

macro_rules! roundf128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function roundf128 in module {}", module_path!());
    };
}

mkfn!{
    roundf128_introspect!();
    # [doc = " Round `x` to the nearest integer, breaking ties away from zero."] # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn roundf128 (x : f128) -> f128 { super :: generic :: round (x) }
}