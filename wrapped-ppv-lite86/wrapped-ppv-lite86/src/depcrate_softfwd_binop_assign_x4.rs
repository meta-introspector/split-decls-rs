// Generated macro for fwd_binop_assign_x4 (macro)
macro_rules! Depcrate_softfwd_binop_assign_x4 {
() => {
// Module: crate::soft
// Provides: {"fwd_binop_assign_x4"}
// Dependencies: {}
macro_rules ! fwd_binop_assign_x4 { ($ trait : ident , $ fn_assign : ident) => { impl < W : $ trait + Copy > $ trait for x4 < W > { # [inline (always)] fn $ fn_assign (& mut self , rhs : Self) { self . 0 [0] .$ fn_assign (rhs . 0 [0]) ; self . 0 [1] .$ fn_assign (rhs . 0 [1]) ; self . 0 [2] .$ fn_assign (rhs . 0 [2]) ; self . 0 [3] .$ fn_assign (rhs . 0 [3]) ; } } } ; }
};
}
