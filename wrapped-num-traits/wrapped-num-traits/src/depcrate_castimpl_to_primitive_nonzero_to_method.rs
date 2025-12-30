// Generated macro for impl_to_primitive_nonzero_to_method (macro)
macro_rules! Depcrate_castimpl_to_primitive_nonzero_to_method {
() => {
// Module: crate::cast
// Provides: {"impl_to_primitive_nonzero_to_method"}
// Dependencies: {}
macro_rules ! impl_to_primitive_nonzero_to_method { ($ SrcT : ident : $ (fn $ method : ident -> $ DstT : ident ;) *) => { $ (# [inline] fn $ method (& self) -> Option <$ DstT > { self . get () .$ method () }) * } }
};
}
