// Generated macro for impl_24 (impl)
macro_rules! Depcrate_dnfimpl_24 {
() => {
// Module: crate::dnf
// Provides: {"impl_24"}
// Dependencies: {}
impl Literal { fn new (expr : CfgExpr) -> Self { match expr { CfgExpr :: Invalid => Self { negate : false , var : None } , CfgExpr :: Atom (atom) => Self { negate : false , var : Some (atom) } , CfgExpr :: Not (expr) => match * expr { CfgExpr :: Invalid => Self { negate : true , var : None } , CfgExpr :: Atom (atom) => Self { negate : true , var : Some (atom) } , _ => unreachable ! ("non-atom {:?}" , expr) , } , CfgExpr :: Any (_) | CfgExpr :: All (_) => unreachable ! ("non-literal {:?}" , expr) , } } }
};
}
