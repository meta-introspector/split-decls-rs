// Generated macro for impl_binop (macro)
macro_rules! Depcrate_x86_64_sse2impl_binop {
() => {
// Module: crate::x86_64::sse2
// Provides: {"impl_binop"}
// Dependencies: {}
macro_rules ! impl_binop { ($ vec : ident , $ trait : ident , $ fn : ident , $ impl_fn : ident) => { impl < S3 , S4 , NI > $ trait for $ vec < S3 , S4 , NI > { type Output = Self ; # [inline (always)] fn $ fn (self , rhs : Self) -> Self :: Output { Self :: new (unsafe { $ impl_fn (self . x , rhs . x) }) } } } ; }
};
}
