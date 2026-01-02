mkuse!{use crate :: support :: Float ;}

macro_rules! fminimum_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fminimum in module {}", module_path!());
    };
}

mkfn!{
    fminimum_introspect!();
    # [inline] pub fn fminimum < F : Float > (x : F , y : F) -> F { let res = if x . is_nan () { x } else if y . is_nan () { y } else if x < y || (x . biteq (F :: NEG_ZERO) && y . is_sign_positive ()) { x } else { y } ; res . canonicalize () }
}