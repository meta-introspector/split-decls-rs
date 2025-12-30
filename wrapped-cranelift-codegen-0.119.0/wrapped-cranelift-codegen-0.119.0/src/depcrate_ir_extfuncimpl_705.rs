// Generated macro for impl_705 (impl)
macro_rules! Depcrate_ir_extfuncimpl_705 {
() => {
// Module: crate::ir::extfunc
// Provides: {"impl_705"}
// Dependencies: {}
impl fmt :: Display for Signature { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "(") ? ; write_list (f , & self . params) ? ; write ! (f , ")") ? ; if ! self . returns . is_empty () { write ! (f , " -> ") ? ; write_list (f , & self . returns) ? ; } write ! (f , " {}" , self . call_conv) } }
};
}
