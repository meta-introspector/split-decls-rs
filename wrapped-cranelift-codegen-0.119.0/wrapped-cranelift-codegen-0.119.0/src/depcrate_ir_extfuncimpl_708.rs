// Generated macro for impl_708 (impl)
macro_rules! Depcrate_ir_extfuncimpl_708 {
() => {
// Module: crate::ir::extfunc
// Provides: {"impl_708"}
// Dependencies: {}
impl fmt :: Display for AbiParam { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "{}" , self . value_type) ? ; match self . extension { ArgumentExtension :: None => { } ArgumentExtension :: Uext => write ! (f , " uext") ? , ArgumentExtension :: Sext => write ! (f , " sext") ? , } if self . purpose != ArgumentPurpose :: Normal { write ! (f , " {}" , self . purpose) ? ; } Ok (()) } }
};
}
