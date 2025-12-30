// Generated macro for impl_to_primitive_wrapping (macro)
macro_rules! Depcrate_castimpl_to_primitive_wrapping {
() => {
// Module: crate::cast
// Provides: {"impl_to_primitive_wrapping"}
// Dependencies: {}
macro_rules ! impl_to_primitive_wrapping { ($ (fn $ method : ident -> $ i : ident ;) *) => { $ (# [inline] fn $ method (& self) -> Option <$ i > { (self . 0) .$ method () }) * } }
};
}
