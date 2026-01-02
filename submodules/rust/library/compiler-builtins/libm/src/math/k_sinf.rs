mkitem!{const S1 : f64 = - 0.166666666416265235595 ;}
mkitem!{const S2 : f64 = 0.0083333293858894631756 ;}
mkitem!{const S3 : f64 = - 0.000198393348360966317347 ;}
mkitem!{const S4 : f64 = 0.0000027183114939898219064 ;}

macro_rules! k_sinf_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function k_sinf in module {}", module_path!());
    };
}

mkfn!{
    k_sinf_introspect!();
    # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub (crate) fn k_sinf (x : f64) -> f32 { let z = x * x ; let w = z * z ; let r = S3 + z * S4 ; let s = z * x ; ((x + s * (S1 + z * S2)) + s * w * r) as f32 }
}