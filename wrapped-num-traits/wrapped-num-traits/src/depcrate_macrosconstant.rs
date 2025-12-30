// Generated macro for constant (macro)
macro_rules! Depcrate_macrosconstant {
() => {
// Module: crate::macros
// Provides: {"constant"}
// Dependencies: {}
macro_rules ! constant { ($ ($ method : ident () -> $ ret : expr ;) *) => { $ (# [inline] fn $ method () -> Self { $ ret }) * } ; }
};
}
