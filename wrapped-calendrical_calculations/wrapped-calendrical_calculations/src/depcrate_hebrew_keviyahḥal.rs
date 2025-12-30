// Generated macro for ḥal (macro)
macro_rules! Depcrate_hebrew_keviyahḥal {
() => {
// Module: crate::hebrew_keviyah
// Provides: {"ḥal"}
// Dependencies: {}
# [doc = " Conveniently create a constant for a ḥalakim (by default in 1-indexed notation). Produces a constant"] # [doc = " that tracks the number of ḥalakim since the beginning of the week"] macro_rules ! ḥal { ($ d : literal -$ h : literal -$ p : literal) => { { const CONSTANT : i32 = (($ d - 1) * 24 + $ h) * 1080 + $ p ; CONSTANT } } ; (0 - indexed $ d : literal -$ h : literal -$ p : literal) => { { const CONSTANT : i32 = ($ d * 24 + $ h) * 1080 + $ p ; CONSTANT } } ; }
};
}
