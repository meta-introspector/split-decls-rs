// Generated macro for impl_26 (impl)
macro_rules! Depcrate_cfgimpl_26 {
() => {
// Module: crate::cfg
// Provides: {"impl_26"}
// Dependencies: {}
impl fmt :: Display for CfgExpr { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { CfgExpr :: Not (ref e) => write ! (f , "not({})" , e) , CfgExpr :: All (ref e) => write ! (f , "all({})" , CommaSep (e)) , CfgExpr :: Any (ref e) => write ! (f , "any({})" , CommaSep (e)) , CfgExpr :: Value (ref e) => write ! (f , "{}" , e) , CfgExpr :: True => write ! (f , "true") , CfgExpr :: False => write ! (f , "false") , } } }
};
}
