// Generated macro for impl_7 (impl)
macro_rules! Depcrate_cfg_exprimpl_7 {
() => {
// Module: crate::cfg_expr
// Provides: {"impl_7"}
// Dependencies: {}
impl fmt :: Display for CfgAtom { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { CfgAtom :: Flag (name) => name . fmt (f) , CfgAtom :: KeyValue { key , value } => write ! (f , "{key} = {value:?}") , } } }
};
}
