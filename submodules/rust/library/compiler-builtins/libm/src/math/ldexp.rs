
macro_rules! ldexpf16_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ldexpf16 in module {}", module_path!());
    };
}

mkfn!{
    ldexpf16_introspect!();
    # [cfg (f16_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn ldexpf16 (x : f16 , n : i32) -> f16 { super :: scalbnf16 (x , n) }
}

macro_rules! ldexpf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ldexpf in module {}", module_path!());
    };
}

mkfn!{
    ldexpf_introspect!();
    # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn ldexpf (x : f32 , n : i32) -> f32 { super :: scalbnf (x , n) }
}

macro_rules! ldexp_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ldexp in module {}", module_path!());
    };
}

mkfn!{
    ldexp_introspect!();
    # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn ldexp (x : f64 , n : i32) -> f64 { super :: scalbn (x , n) }
}

macro_rules! ldexpf128_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ldexpf128 in module {}", module_path!());
    };
}

mkfn!{
    ldexpf128_introspect!();
    # [cfg (f128_enabled)] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn ldexpf128 (x : f128 , n : i32) -> f128 { super :: scalbnf128 (x , n) }
}