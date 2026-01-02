mkitem!{const C0 : f64 = - 0.499999997251031003120 ;}
mkitem!{const C1 : f64 = 0.0416666233237390631894 ;}
mkitem!{const C2 : f64 = - 0.00138867637746099294692 ;}
mkitem!{const C3 : f64 = 0.0000243904487962774090654 ;}

macro_rules! k_cosf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function k_cosf in module {}", module_path!());
    };
}

mkfn!{
    k_cosf_introspect!();
    # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub (crate) fn k_cosf (x : f64) -> f32 { let z = x * x ; let w = z * z ; let r = C2 + z * C3 ; (((1.0 + z * C0) + w * C1) + (w * z) * r) as f32 }
}