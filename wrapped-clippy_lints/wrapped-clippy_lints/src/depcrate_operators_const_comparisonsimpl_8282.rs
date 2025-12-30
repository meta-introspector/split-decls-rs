// Generated macro for impl_8282 (impl)
macro_rules! Depcrate_operators_const_comparisonsimpl_8282 {
() => {
// Module: crate::operators::const_comparisons
// Provides: {"impl_8282"}
// Dependencies: {}
impl CmpOp { fn reverse (self) -> Self { match self { CmpOp :: Lt => CmpOp :: Gt , CmpOp :: Le => CmpOp :: Ge , CmpOp :: Ge => CmpOp :: Le , CmpOp :: Gt => CmpOp :: Lt , } } fn direction (self) -> CmpOpDirection { match self { CmpOp :: Lt => CmpOpDirection :: Lesser , CmpOp :: Le => CmpOpDirection :: Lesser , CmpOp :: Ge => CmpOpDirection :: Greater , CmpOp :: Gt => CmpOpDirection :: Greater , } } }
};
}
