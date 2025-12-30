// Generated macro for forward (macro)
macro_rules! Depcrate_complex_floatforward {
() => {
// Module: crate::complex_float
// Provides: {"forward"}
// Dependencies: {}
macro_rules ! forward { ($ ($ base : ident :: $ method : ident (self $ (, $ arg : ident : $ ty : ty) *) -> $ ret : ty ;) *) => { $ (# [inline] fn $ method (self $ (, $ arg : $ ty) *) -> $ ret { $ base ::$ method (self $ (, $ arg) *) }) * } ; }
};
}
