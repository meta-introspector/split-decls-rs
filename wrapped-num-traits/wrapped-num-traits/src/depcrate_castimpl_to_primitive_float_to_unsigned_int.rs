// Generated macro for impl_to_primitive_float_to_unsigned_int (macro)
macro_rules! Depcrate_castimpl_to_primitive_float_to_unsigned_int {
() => {
// Module: crate::cast
// Provides: {"impl_to_primitive_float_to_unsigned_int"}
// Dependencies: {}
macro_rules ! impl_to_primitive_float_to_unsigned_int { ($ f : ident : $ (fn $ method : ident -> $ u : ident ;) *) => { $ (# [inline] fn $ method (& self) -> Option <$ u > { if size_of ::<$ f > () > size_of ::<$ u > () { const MAX_P1 : $ f = $ u :: MAX as $ f + 1.0 ; if * self > - 1.0 && * self < MAX_P1 { return Some (float_to_int_unchecked ! (* self => $ u)) ; } } else { const MAX_P1 : $ f = $ u :: MAX as $ f ; if * self > - 1.0 && * self < MAX_P1 { return Some (float_to_int_unchecked ! (* self => $ u)) ; } } None }) * } }
};
}
