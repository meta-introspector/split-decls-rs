// Generated macro for make_nnf_neg (function)
macro_rules! Depcrate_dnfmake_nnf_neg {
() => {
// Module: crate::dnf
// Provides: {"make_nnf_neg"}
// Dependencies: {}
fn make_nnf_neg (operand : & CfgExpr) -> CfgExpr { match operand { CfgExpr :: Invalid => CfgExpr :: Not (Box :: new (CfgExpr :: Invalid)) , CfgExpr :: Atom (atom) => CfgExpr :: Not (Box :: new (CfgExpr :: Atom (atom . clone ()))) , CfgExpr :: Not (expr) => make_nnf (expr) , CfgExpr :: Any (inner) => CfgExpr :: All (inner . iter () . map (make_nnf_neg) . collect ()) , CfgExpr :: All (inner) => CfgExpr :: Any (inner . iter () . map (make_nnf_neg) . collect ()) , } }
};
}
