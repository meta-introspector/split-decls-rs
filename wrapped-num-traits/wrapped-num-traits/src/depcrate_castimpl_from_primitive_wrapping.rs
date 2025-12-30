// Generated macro for impl_from_primitive_wrapping (macro)
macro_rules! Depcrate_castimpl_from_primitive_wrapping {
() => {
// Module: crate::cast
// Provides: {"impl_from_primitive_wrapping"}
// Dependencies: {}
macro_rules ! impl_from_primitive_wrapping { ($ (fn $ method : ident ($ i : ident) ;) *) => { $ (# [inline] fn $ method (n : $ i) -> Option < Self > { T ::$ method (n) . map (Wrapping) }) * } }
};
}
