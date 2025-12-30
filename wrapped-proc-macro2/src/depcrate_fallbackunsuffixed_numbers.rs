// Generated macro for unsuffixed_numbers (macro)
macro_rules! Depcrate_fallbackunsuffixed_numbers {
() => {
// Module: crate::fallback
// Provides: {"unsuffixed_numbers"}
// Dependencies: {}
macro_rules ! unsuffixed_numbers { ($ ($ name : ident => $ kind : ident ,) *) => ($ (pub (crate) fn $ name (n : $ kind) -> Literal { Literal :: _new (n . to_string ()) }) *) }
};
}
