mkuse!{use crate :: support :: Float ;}

macro_rules! fminimum_num_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fminimum_num in module {}", module_path!());
    };
}

mkfn!{
    fminimum_num_introspect!();
    # [inline] pub fn fminimum_num < F : Float > (x : F , y : F) -> F { let res = if x > y || x . is_nan () { y } else if y > x || y . is_nan () { x } else if x . is_sign_positive () { y } else { x } ; res . canonicalize () }
}