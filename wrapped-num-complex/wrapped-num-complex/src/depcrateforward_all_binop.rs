// Generated macro for forward_all_binop (macro)
macro_rules! Depcrateforward_all_binop {
() => {
// Module: crate
// Provides: {"forward_all_binop"}
// Dependencies: {}
macro_rules ! forward_all_binop { (impl $ imp : ident , $ method : ident) => { forward_ref_ref_binop ! (impl $ imp , $ method) ; forward_ref_val_binop ! (impl $ imp , $ method) ; forward_val_ref_binop ! (impl $ imp , $ method) ; } ; }
};
}
