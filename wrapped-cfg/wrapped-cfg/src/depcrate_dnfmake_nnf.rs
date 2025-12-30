// Generated macro for make_nnf (function)
macro_rules! Depcrate_dnfmake_nnf {
() => {
// Module: crate::dnf
// Provides: {"make_nnf"}
// Dependencies: {}
fn make_nnf (expr : & CfgExpr) -> CfgExpr { match expr { CfgExpr :: Invalid | CfgExpr :: Atom (_) => expr . clone () , CfgExpr :: Any (expr) => CfgExpr :: Any (expr . iter () . map (make_nnf) . collect ()) , CfgExpr :: All (expr) => CfgExpr :: All (expr . iter () . map (make_nnf) . collect ()) , CfgExpr :: Not (operand) => make_nnf_neg (operand) , } }
};
}
