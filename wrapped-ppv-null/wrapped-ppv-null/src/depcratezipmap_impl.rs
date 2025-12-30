// Generated macro for zipmap_impl (macro)
macro_rules! Depcratezipmap_impl {
() => {
// Module: crate
// Provides: {"zipmap_impl"}
// Dependencies: {}
macro_rules ! zipmap_impl { ($ vec : ident , $ word : ident , $ trait : ident , $ fn : ident , $ impl_fn : ident) => { impl $ trait for $ vec { type Output = Self ; # [inline (always)] fn $ fn (self , rhs : Self) -> Self :: Output { self . zipmap (rhs , $ word ::$ impl_fn) } } } ; ($ vec : ident , $ word : ident , $ trait : ident , $ fn : ident) => { zipmap_impl ! ($ vec , $ word , $ trait , $ fn , $ fn) ; } ; }
};
}
