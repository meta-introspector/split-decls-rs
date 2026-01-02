mkuse!{use super :: exp ;}
mkitem!{const K : i32 = 2043 ;}

macro_rules! k_expo2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function k_expo2 in module {}", module_path!());
    };
}

mkfn!{
    k_expo2_introspect!();
    # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub (crate) fn k_expo2 (x : f64) -> f64 { let k_ln2 = f64 :: from_bits (0x40962066151add8b) ; let scale = f64 :: from_bits (((((0x3ff + K / 2) as u32) << 20) as u64) << 32) ; exp (x - k_ln2) * scale * scale }
}