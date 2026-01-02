mkuse!{use crate :: support :: Float ;}

macro_rules! copysign_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function copysign in module {}", module_path!());
    };
}

mkfn!{
    copysign_introspect!();
    # [doc = " Copy the sign of `y` to `x`."] # [inline] pub fn copysign < F : Float > (x : F , y : F) -> F { let mut ux = x . to_bits () ; let uy = y . to_bits () ; ux &= ! F :: SIGN_MASK ; ux |= uy & F :: SIGN_MASK ; F :: from_bits (ux) }
}