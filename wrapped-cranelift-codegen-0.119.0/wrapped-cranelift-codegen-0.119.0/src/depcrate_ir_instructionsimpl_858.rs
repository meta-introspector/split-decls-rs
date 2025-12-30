// Generated macro for impl_858 (impl)
macro_rules! Depcrate_ir_instructionsimpl_858 {
() => {
// Module: crate::ir::instructions
// Provides: {"impl_858"}
// Dependencies: {}
impl < 'a > Display for DisplayBlockCall < 'a > { fn fmt (& self , f : & mut Formatter < '_ >) -> fmt :: Result { write ! (f , "{}" , self . block . block (& self . pool)) ? ; let args = self . block . args_slice (& self . pool) ; if ! args . is_empty () { write ! (f , "(") ? ; for (ix , arg) in args . iter () . enumerate () { if ix > 0 { write ! (f , ", ") ? ; } write ! (f , "{arg}") ? ; } write ! (f , ")") ? ; } Ok (()) } }
};
}
