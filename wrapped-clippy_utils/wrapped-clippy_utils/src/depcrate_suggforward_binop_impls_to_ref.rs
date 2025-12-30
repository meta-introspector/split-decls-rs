// Generated macro for forward_binop_impls_to_ref (macro)
macro_rules! Depcrate_suggforward_binop_impls_to_ref {
() => {
// Module: crate::sugg
// Provides: {"forward_binop_impls_to_ref"}
// Dependencies: {}
# [doc = " Copied from the rust standard library, and then edited"] macro_rules ! forward_binop_impls_to_ref { (impl $ imp : ident , $ method : ident for $ t : ty , type Output = $ o : ty) => { impl $ imp <$ t > for &$ t { type Output = $ o ; fn $ method (self , other : $ t) -> $ o { $ imp ::$ method (self , & other) } } impl $ imp <&$ t > for $ t { type Output = $ o ; fn $ method (self , other : &$ t) -> $ o { $ imp ::$ method (& self , other) } } impl $ imp for $ t { type Output = $ o ; fn $ method (self , other : $ t) -> $ o { $ imp ::$ method (& self , & other) } } } ; }
};
}
