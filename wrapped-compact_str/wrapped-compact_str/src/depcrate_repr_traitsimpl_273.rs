// Generated macro for impl_273 (impl)
macro_rules! Depcrate_repr_traitsimpl_273 {
() => {
// Module: crate::repr::traits
// Provides: {"impl_273"}
// Dependencies: {}
impl IntoRepr for char { # [inline] fn into_repr (self) -> Result < Repr , ToCompactStringError > { let mut buf = [0_u8 ; 4] ; let s = self . encode_utf8 (& mut buf) ; match s . len () { 1 ..= 4 => () , _ => unsafe { unreachable_unchecked () } , } Ok (Repr :: new (s) ?) } }
};
}
