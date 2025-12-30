// Generated macro for forward_js_unop (macro)
macro_rules! Depcrateforward_js_unop {
() => {
// Module: crate
// Provides: {"forward_js_unop"}
// Dependencies: {}
macro_rules ! forward_js_unop { (impl $ imp : ident , $ method : ident for $ t : ty) => { impl $ imp for &$ t { type Output = $ t ; # [inline] fn $ method (self) -> Self :: Output { $ imp ::$ method (JsValue :: as_ref (self)) . unchecked_into () } } forward_deref_unop ! (impl $ imp , $ method for $ t) ; } ; }
};
}
