mkuse!{use crate :: support :: Float ;}

macro_rules! fmaximum_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmaximum in module {}", module_path!());
    };
}

mkfn!{
    fmaximum_introspect!();
    # [inline] pub fn fmaximum < F : Float > (x : F , y : F) -> F { let res = if x . is_nan () { x } else if y . is_nan () { y } else if x > y || (y . biteq (F :: NEG_ZERO) && x . is_sign_positive ()) { x } else { y } ; res . canonicalize () }
}