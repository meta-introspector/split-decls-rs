// Generated macro for impl_to_primitive_int_to_int (macro)
macro_rules! Depcrate_castimpl_to_primitive_int_to_int {
() => {
// Module: crate::cast
// Provides: {"impl_to_primitive_int_to_int"}
// Dependencies: {}
macro_rules ! impl_to_primitive_int_to_int { ($ SrcT : ident : $ (fn $ method : ident -> $ DstT : ident ;) *) => { $ (# [inline] fn $ method (& self) -> Option <$ DstT > { let min = $ DstT :: MIN as $ SrcT ; let max = $ DstT :: MAX as $ SrcT ; if size_of ::<$ SrcT > () <= size_of ::<$ DstT > () || (min <= * self && * self <= max) { Some (* self as $ DstT) } else { None } }) * } }
};
}
