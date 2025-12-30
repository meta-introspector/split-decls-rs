// Generated macro for forward_ref_ref_binop (macro)
macro_rules! Depcrateforward_ref_ref_binop {
() => {
// Module: crate
// Provides: {"forward_ref_ref_binop"}
// Dependencies: {}
macro_rules ! forward_ref_ref_binop { (impl $ imp : ident , $ method : ident) => { impl <'a , 'b , T : Clone + Num > $ imp <&'b Complex < T >> for &'a Complex < T > { type Output = Complex < T >; # [inline] fn $ method (self , other : & Complex < T >) -> Self :: Output { self . clone () .$ method (other . clone ()) } } } ; }
};
}
