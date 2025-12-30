// Generated macro for fwd_unop_x4 (macro)
macro_rules! Depcrate_softfwd_unop_x4 {
() => {
// Module: crate::soft
// Provides: {"fwd_unop_x4"}
// Dependencies: {}
macro_rules ! fwd_unop_x4 { ($ fn : ident) => { # [inline (always)] fn $ fn (self) -> Self { x4 ([self . 0 [0] .$ fn () , self . 0 [1] .$ fn () , self . 0 [2] .$ fn () , self . 0 [3] .$ fn () ,]) } } ; }
};
}
