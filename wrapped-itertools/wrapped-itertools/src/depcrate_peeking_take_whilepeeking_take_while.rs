// Generated macro for peeking_take_while (function)
macro_rules! Depcrate_peeking_take_whilepeeking_take_while {
() => {
// Module: crate::peeking_take_while
// Provides: {"peeking_take_while"}
// Dependencies: {}
# [doc = " Create a `PeekingTakeWhile`"] pub fn peeking_take_while < I , F > (iter : & mut I , f : F) -> PeekingTakeWhile < '_ , I , F > where I : Iterator , { PeekingTakeWhile { iter , f } }
};
}
