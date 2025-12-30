// Generated macro for impl_to_primitive_uint_to_int (macro)
macro_rules! Depcrate_castimpl_to_primitive_uint_to_int {
() => {
// Module: crate::cast
// Provides: {"impl_to_primitive_uint_to_int"}
// Dependencies: {}
macro_rules ! impl_to_primitive_uint_to_int { ($ SrcT : ident : $ (fn $ method : ident -> $ DstT : ident ;) *) => { $ (# [inline] fn $ method (& self) -> Option <$ DstT > { let max = $ DstT :: MAX as $ SrcT ; if size_of ::<$ SrcT > () < size_of ::<$ DstT > () || * self <= max { Some (* self as $ DstT) } else { None } }) * } }
};
}
