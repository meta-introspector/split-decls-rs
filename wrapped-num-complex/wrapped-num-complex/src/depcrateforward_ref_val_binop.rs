// Generated macro for forward_ref_val_binop (macro)
macro_rules! Depcrateforward_ref_val_binop {
() => {
// Module: crate
// Provides: {"forward_ref_val_binop"}
// Dependencies: {}
macro_rules ! forward_ref_val_binop { (impl $ imp : ident , $ method : ident) => { impl <'a , T : Clone + Num > $ imp < Complex < T >> for &'a Complex < T > { type Output = Complex < T >; # [inline] fn $ method (self , other : Complex < T >) -> Self :: Output { self . clone () .$ method (other) } } } ; }
};
}
