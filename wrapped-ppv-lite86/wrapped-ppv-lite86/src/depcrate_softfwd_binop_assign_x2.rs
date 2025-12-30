// Generated macro for fwd_binop_assign_x2 (macro)
macro_rules! Depcrate_softfwd_binop_assign_x2 {
() => {
// Module: crate::soft
// Provides: {"fwd_binop_assign_x2"}
// Dependencies: {}
macro_rules ! fwd_binop_assign_x2 { ($ trait : ident , $ fn_assign : ident) => { impl < W : $ trait + Copy , G > $ trait for x2 < W , G > { # [inline (always)] fn $ fn_assign (& mut self , rhs : Self) { (self . 0 [0]) .$ fn_assign (rhs . 0 [0]) ; (self . 0 [1]) .$ fn_assign (rhs . 0 [1]) ; } } } ; }
};
}
