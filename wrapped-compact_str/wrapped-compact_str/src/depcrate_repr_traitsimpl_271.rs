// Generated macro for impl_271 (impl)
macro_rules! Depcrate_repr_traitsimpl_271 {
() => {
// Module: crate::repr::traits
// Provides: {"impl_271"}
// Dependencies: {}
impl IntoRepr for f64 { # [inline] fn into_repr (self) -> Result < Repr , ToCompactStringError > { let mut buf = ryu :: Buffer :: new () ; let s = buf . format (self) ; Ok (Repr :: new (s) ?) } }
};
}
