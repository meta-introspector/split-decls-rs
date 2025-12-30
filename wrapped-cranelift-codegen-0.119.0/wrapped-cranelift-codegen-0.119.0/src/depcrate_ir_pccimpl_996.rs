// Generated macro for impl_996 (impl)
macro_rules! Depcrate_ir_pccimpl_996 {
() => {
// Module: crate::ir::pcc
// Provides: {"impl_996"}
// Dependencies: {}
impl fmt :: Display for Expr { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { write ! (f , "{}" , self . base) ? ; match self . offset { offset if offset > 0 && self . base . is_some () => write ! (f , "+{offset:#x}") , offset if offset > 0 => write ! (f , "{offset:#x}") , offset if offset < 0 => { let negative_offset = - i128 :: from (offset) ; write ! (f , "-{negative_offset:#x}") } 0 if self . base . is_some () => Ok (()) , 0 => write ! (f , "0") , _ => unreachable ! () , } } }
};
}
