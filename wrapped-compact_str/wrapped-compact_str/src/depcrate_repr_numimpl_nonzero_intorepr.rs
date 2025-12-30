// Generated macro for impl_NonZero_IntoRepr (macro)
macro_rules! Depcrate_repr_numimpl_NonZero_IntoRepr {
() => {
// Module: crate::repr::num
// Provides: {"impl_NonZero_IntoRepr"}
// Dependencies: {}
# [doc = " Defines the implementation of [`IntoRepr`] for NonZero integer types"] macro_rules ! impl_NonZero_IntoRepr { ($ t : path) => { impl IntoRepr for $ t { # [inline] fn into_repr (self) -> Result < Repr , ToCompactStringError > { self . get () . into_repr () } } } ; }
};
}
