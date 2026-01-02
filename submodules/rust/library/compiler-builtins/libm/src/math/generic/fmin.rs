mkuse!{use crate :: support :: Float ;}

macro_rules! fmin_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmin in module {}", module_path!());
    };
}

mkfn!{
    fmin_introspect!();
    # [inline] pub fn fmin < F : Float > (x : F , y : F) -> F { let res = if y . is_nan () || x < y { x } else { y } ; res . canonicalize () }
}