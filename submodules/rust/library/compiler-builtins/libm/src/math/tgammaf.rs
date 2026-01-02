mkuse!{use super :: tgamma ;}

macro_rules! tgammaf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function tgammaf in module {}", module_path!());
    };
}

mkfn!{
    tgammaf_introspect!();
    # [doc = " The [Gamma function](https://en.wikipedia.org/wiki/Gamma_function) (f32)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn tgammaf (x : f32) -> f32 { tgamma (x as f64) as f32 }
}