// Generated macro for fwd_binop_x2 (macro)
macro_rules! Depcrate_softfwd_binop_x2 {
() => {
// Module: crate::soft
// Provides: {"fwd_binop_x2"}
// Dependencies: {}
macro_rules ! fwd_binop_x2 { ($ trait : ident , $ fn : ident) => { impl < W : $ trait + Copy , G > $ trait for x2 < W , G > { type Output = x2 < W :: Output , G >; # [inline (always)] fn $ fn (self , rhs : Self) -> Self :: Output { x2 :: new ([self . 0 [0] .$ fn (rhs . 0 [0]) , self . 0 [1] .$ fn (rhs . 0 [1])]) } } } ; }
};
}
