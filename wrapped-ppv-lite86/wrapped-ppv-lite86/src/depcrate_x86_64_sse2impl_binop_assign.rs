// Generated macro for impl_binop_assign (macro)
macro_rules! Depcrate_x86_64_sse2impl_binop_assign {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_binop_assign"}
// Dependencies: {}
macro_rules ! impl_binop_assign { ($ vec : ident , $ trait : ident , $ fn_assign : ident , $ fn : ident) => { impl < S3 , S4 , NI > $ trait for $ vec < S3 , S4 , NI > where $ vec < S3 , S4 , NI >: Copy , { # [inline (always)] fn $ fn_assign (& mut self , rhs : Self) { * self = self .$ fn (rhs) ; } } } ; }
};
}
