// Generated macro for suffixed_numbers (macro)
macro_rules! Depcrate_fallbacksuffixed_numbers {
() => {
// Module: crate::fallback
// Provides: {"suffixed_numbers"}
// Dependencies: {}
macro_rules ! suffixed_numbers { ($ ($ name : ident => $ kind : ident ,) *) => ($ (pub (crate) fn $ name (n : $ kind) -> Literal { Literal :: _new (format ! (concat ! ("{}" , stringify ! ($ kind)) , n)) }) *) }
};
}
