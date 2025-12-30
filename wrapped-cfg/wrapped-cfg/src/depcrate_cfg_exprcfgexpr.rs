// Generated macro for CfgExpr (enum)
macro_rules! Depcrate_cfg_exprCfgExpr {
() => {
// Module: crate::cfg_expr
// Provides: {"CfgExpr"}
// Dependencies: {}
# [derive (Debug , Clone , PartialEq , Eq , Hash)] # [cfg_attr (test , derive (arbitrary :: Arbitrary))] pub enum CfgExpr { Invalid , Atom (CfgAtom) , All (Box < [CfgExpr] >) , Any (Box < [CfgExpr] >) , Not (Box < CfgExpr >) , }
};
}
