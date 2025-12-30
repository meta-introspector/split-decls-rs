// Generated macro for impl_from_primitive (macro)
macro_rules! Depcrate_castimpl_from_primitive {
() => {
// Module: crate::cast
// Provides: {"impl_from_primitive"}
// Dependencies: {}
macro_rules ! impl_from_primitive { ($ ty : ty , $ from_xx : ident) => { # [inline] fn $ from_xx (n : $ ty) -> Option < Self > { Some (Complex { re : T ::$ from_xx (n) ?, im : T :: zero () , }) } } ; }
};
}
