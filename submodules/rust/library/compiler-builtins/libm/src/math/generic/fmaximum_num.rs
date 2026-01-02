mkuse!{use crate :: support :: Float ;}

macro_rules! fmaximum_num_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmaximum_num in module {}", module_path!());
    };
}

mkfn!{
    fmaximum_num_introspect!();
    # [inline] pub fn fmaximum_num < F : Float > (x : F , y : F) -> F { let res = if x > y || y . is_nan () { x } else if y > x || x . is_nan () { y } else if x . is_sign_positive () { x } else { y } ; res . canonicalize () }
}