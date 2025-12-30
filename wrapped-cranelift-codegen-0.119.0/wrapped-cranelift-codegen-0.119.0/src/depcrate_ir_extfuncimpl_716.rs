// Generated macro for impl_716 (impl)
macro_rules! Depcrate_ir_extfuncimpl_716 {
() => {
// Module: crate::ir::extfunc
// Provides: {"impl_716"}
// Dependencies: {}
impl < 'a > fmt :: Display for DisplayableExtFuncData < 'a > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { if self . ext_func . colocated { write ! (f , "colocated ") ? ; } write ! (f , "{} {}" , self . ext_func . name . display (self . params) , self . ext_func . signature) } }
};
}
