mkitem!{const S1 : f64 = - 1.66666666666666324348e-01 ;}
mkitem!{const S2 : f64 = 8.33333333332248946124e-03 ;}
mkitem!{const S3 : f64 = - 1.98412698298579493134e-04 ;}
mkitem!{const S4 : f64 = 2.75573137070700676789e-06 ;}
mkitem!{const S5 : f64 = - 2.50507602534068634195e-08 ;}
mkitem!{const S6 : f64 = 1.58969099521155010221e-10 ;}

macro_rules! k_sin_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function k_sin in module {}", module_path!());
    };
}

mkfn!{
    k_sin_introspect!();
    # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub (crate) fn k_sin (x : f64 , y : f64 , iy : i32) -> f64 { let z = x * x ; let w = z * z ; let r = S2 + z * (S3 + z * S4) + z * w * (S5 + z * S6) ; let v = z * x ; if iy == 0 { x + v * (S1 + z * r) } else { x - ((z * (0.5 * y - v * r) - y) - v * S1) } }
}