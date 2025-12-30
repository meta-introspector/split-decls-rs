// Generated macro for generate (macro)
macro_rules! Depcrate_symgenerate {
() => {
// Module: crate::sym
// Provides: {"generate"}
// Dependencies: {}
macro_rules ! generate { ($ ($ name : ident $ (: $ value : literal) ? ,) *) => { # [doc = " To be supplied to `rustc_interface::Config`"] pub const EXTRA_SYMBOLS : & [& str] = & [$ (val ! ($ name $ ($ value) ?) ,) *] ; $ (pub const $ name : rustc_span :: Symbol = rustc_span :: Symbol :: new (PREDEFINED_SYMBOLS_COUNT + $ { index () }) ;) * } ; }
};
}
