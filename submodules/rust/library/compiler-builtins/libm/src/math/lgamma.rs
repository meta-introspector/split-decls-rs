mkuse!{use super :: lgamma_r ;}

macro_rules! lgamma_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lgamma in module {}", module_path!());
    };
}

mkfn!{
    lgamma_introspect!();
    # [doc = " The natural logarithm of the"] # [doc = " [Gamma function](https://en.wikipedia.org/wiki/Gamma_function) (f64)."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn lgamma (x : f64) -> f64 { lgamma_r (x) . 0 }
}