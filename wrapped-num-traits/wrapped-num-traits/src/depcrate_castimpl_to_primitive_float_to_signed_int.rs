// Generated macro for impl_to_primitive_float_to_signed_int (macro)
macro_rules! Depcrate_castimpl_to_primitive_float_to_signed_int {
() => {
// Module: crate::cast
// Provides: {"impl_to_primitive_float_to_signed_int"}
// Dependencies: {}
macro_rules ! impl_to_primitive_float_to_signed_int { ($ f : ident : $ (fn $ method : ident -> $ i : ident ;) *) => { $ (# [inline] fn $ method (& self) -> Option <$ i > { if size_of ::<$ f > () > size_of ::<$ i > () { const MIN_M1 : $ f = $ i :: MIN as $ f - 1.0 ; const MAX_P1 : $ f = $ i :: MAX as $ f + 1.0 ; if * self > MIN_M1 && * self < MAX_P1 { return Some (float_to_int_unchecked ! (* self => $ i)) ; } } else { const MIN : $ f = $ i :: MIN as $ f ; const MAX_P1 : $ f = $ i :: MAX as $ f ; if * self >= MIN && * self < MAX_P1 { return Some (float_to_int_unchecked ! (* self => $ i)) ; } } None }) * } }
};
}
