mkuse!{use crate :: support :: Float ;}

macro_rules! fdim_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function fdim in module {}", module_path!());
    };
}

mkfn!{
    fdim_introspect!();
    # [inline] pub fn fdim < F : Float > (x : F , y : F) -> F { if x <= y { F :: ZERO } else { x - y } }
}