mkuse!{use crate :: support :: Float ;}

macro_rules! fabs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fabs in module {}", module_path!());
    };
}

mkfn!{
    fabs_introspect!();
    # [doc = " Absolute value."] # [inline] pub fn fabs < F : Float > (x : F) -> F { let abs_mask = ! F :: SIGN_MASK ; F :: from_bits (x . to_bits () & abs_mask) }
}