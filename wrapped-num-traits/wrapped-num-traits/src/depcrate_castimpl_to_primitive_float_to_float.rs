// Generated macro for impl_to_primitive_float_to_float (macro)
macro_rules! Depcrate_castimpl_to_primitive_float_to_float {
() => {
// Module: crate::cast
// Provides: {"impl_to_primitive_float_to_float"}
// Dependencies: {}
macro_rules ! impl_to_primitive_float_to_float { ($ SrcT : ident : $ (fn $ method : ident -> $ DstT : ident ;) *) => { $ (# [inline] fn $ method (& self) -> Option <$ DstT > { Some (* self as $ DstT) }) * } }
};
}
