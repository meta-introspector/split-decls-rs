// Generated macro for impl_270 (impl)
macro_rules! Depcrate_repr_traitsimpl_270 {
() => {
// Module: crate::repr::traits
// Provides: {"impl_270"}
// Dependencies: {}
impl IntoRepr for f32 { # [inline] fn into_repr (self) -> Result < Repr , ToCompactStringError > { let mut buf = ryu :: Buffer :: new () ; let s = buf . format (self) ; Ok (Repr :: new (s) ?) } }
};
}
