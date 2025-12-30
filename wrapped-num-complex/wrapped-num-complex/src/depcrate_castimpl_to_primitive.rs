// Generated macro for impl_to_primitive (macro)
macro_rules! Depcrate_castimpl_to_primitive {
() => {
// Module: crate::cast
// Provides: {"impl_to_primitive"}
// Dependencies: {}
macro_rules ! impl_to_primitive { ($ ty : ty , $ to : ident) => { # [inline] fn $ to (& self) -> Option <$ ty > { if self . im . is_zero () { self . re .$ to () } else { None } } } ; }
};
}
