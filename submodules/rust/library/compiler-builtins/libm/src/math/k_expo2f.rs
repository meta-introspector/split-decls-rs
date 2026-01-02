mkuse!{use super :: expf ;}
mkitem!{const K : i32 = 235 ;}

macro_rules! k_expo2f_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function k_expo2f in module {}", module_path!());
    };
}

mkfn!{
    k_expo2f_introspect!();
    # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub (crate) fn k_expo2f (x : f32) -> f32 { let k_ln2 = f32 :: from_bits (0x4322e3bc) ; let scale = f32 :: from_bits (((0x7f + K / 2) as u32) << 23) ; expf (x - k_ln2) * scale * scale }
}