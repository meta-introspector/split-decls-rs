// Generated macro for impl_994 (impl)
macro_rules! Depcrate_ir_pccimpl_994 {
() => {
// Module: crate::ir::pcc
// Provides: {"impl_994"}
// Dependencies: {}
impl fmt :: Display for BaseExpr { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { BaseExpr :: None => Ok (()) , BaseExpr :: Max => write ! (f , "max") , BaseExpr :: GlobalValue (gv) => write ! (f , "{gv}") , BaseExpr :: Value (value) => write ! (f , "{value}") , } } }
};
}
