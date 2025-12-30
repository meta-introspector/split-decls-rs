// Generated macro for impl_730 (impl)
macro_rules! Depcrate_ir_extnameimpl_730 {
() => {
// Module: crate::ir::extname
// Provides: {"impl_730"}
// Dependencies: {}
impl fmt :: Display for UserFuncName { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { UserFuncName :: User (user) => user . fmt (f) , UserFuncName :: Testcase (testcase) => testcase . fmt (f) , } } }
};
}
