// Generated macro for forward_val_ref_binop (macro)
macro_rules! Depcrateforward_val_ref_binop {
() => {
// Module: crate
// Provides: {"forward_val_ref_binop"}
// Dependencies: {}
macro_rules ! forward_val_ref_binop { (impl $ imp : ident , $ method : ident) => { impl <'a , T : Clone + Num > $ imp <&'a Complex < T >> for Complex < T > { type Output = Complex < T >; # [inline] fn $ method (self , other : & Complex < T >) -> Self :: Output { self .$ method (other . clone ()) } } } ; }
};
}
