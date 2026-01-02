mkuse!{use super :: lgammaf_r ;}

macro_rules! lgammaf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lgammaf in module {}", module_path!());
    };
}

mkfn!{
    lgammaf_introspect!();
    # [doc = " The natural logarithm of the"] # [doc = " [Gamma function](https://en.wikipedia.org/wiki/Gamma_function) (f32)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn lgammaf (x : f32) -> f32 { lgammaf_r (x) . 0 }
}