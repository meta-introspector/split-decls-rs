// Generated macro for forward_ref (macro)
macro_rules! Depcrate_complex_floatforward_ref {
() => {
// Module: crate::complex_float
// Provides: {"forward_ref"}
// Dependencies: {}
macro_rules ! forward_ref { ($ (Self :: $ method : ident (& self $ (, $ arg : ident : $ ty : ty) *) -> $ ret : ty ;) *) => { $ (# [inline] fn $ method (self $ (, $ arg : $ ty) *) -> $ ret { Self ::$ method (& self $ (, $ arg) *) }) * } ; }
};
}
