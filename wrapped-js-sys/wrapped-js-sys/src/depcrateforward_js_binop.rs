// Generated macro for forward_js_binop (macro)
macro_rules! Depcrateforward_js_binop {
() => {
// Module: crate
// Provides: {"forward_js_binop"}
// Dependencies: {}
macro_rules ! forward_js_binop { (impl $ imp : ident , $ method : ident for $ t : ty) => { impl $ imp <&$ t > for &$ t { type Output = $ t ; # [inline] fn $ method (self , other : &$ t) -> Self :: Output { $ imp ::$ method (JsValue :: as_ref (self) , JsValue :: as_ref (other)) . unchecked_into () } } forward_deref_binop ! (impl $ imp , $ method for $ t) ; } ; }
};
}
