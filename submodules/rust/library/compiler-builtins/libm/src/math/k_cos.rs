mkitem!{const C1 : f64 = 4.16666666666666019037e-02 ;}
mkitem!{const C2 : f64 = - 1.38888888888741095749e-03 ;}
mkitem!{const C3 : f64 = 2.48015872894767294178e-05 ;}
mkitem!{const C4 : f64 = - 2.75573143513906633035e-07 ;}
mkitem!{const C5 : f64 = 2.08757232129817482790e-09 ;}
mkitem!{const C6 : f64 = - 1.13596475577881948265e-11 ;}

macro_rules! k_cos_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function k_cos in module {}", module_path!());
    };
}

mkfn!{
    k_cos_introspect!();
    # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub (crate) fn k_cos (x : f64 , y : f64) -> f64 { let z = x * x ; let w = z * z ; let r = z * (C1 + z * (C2 + z * C3)) + w * w * (C4 + z * (C5 + z * C6)) ; let hz = 0.5 * z ; let w = 1.0 - hz ; w + (((1.0 - w) - hz) + (z * r - x * y)) }
}