mkuse!{use crate :: support :: Float ;}

macro_rules! fmax_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fmax in module {}", module_path!());
    };
}

mkfn!{
    fmax_introspect!();
    # [inline] pub fn fmax < F : Float > (x : F , y : F) -> F { let res = if x . is_nan () || x < y { y } else { x } ; res . canonicalize () }
}