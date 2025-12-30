// Generated macro for impl_228 (impl)
macro_rules! Depcrate_repr_numimpl_228 {
() => {
// Module: crate::repr::num
// Provides: {"impl_228"}
// Dependencies: {}
impl IntoRepr for i128 { # [inline] fn into_repr (self) -> Result < Repr , ToCompactStringError > { let mut buffer = itoa :: Buffer :: new () ; Ok (Repr :: new (buffer . format (self)) ?) } }
};
}
