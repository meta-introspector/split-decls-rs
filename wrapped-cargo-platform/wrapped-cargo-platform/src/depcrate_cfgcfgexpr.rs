// Generated macro for CfgExpr (enum)
macro_rules! Depcrate_cfgCfgExpr {
() => {
// Module: crate::cfg
// Provides: {"CfgExpr"}
// Dependencies: {}
# [doc = " A cfg expression."] # [derive (Eq , PartialEq , Hash , Ord , PartialOrd , Clone , Debug)] pub enum CfgExpr { Not (Box < CfgExpr >) , All (Vec < CfgExpr >) , Any (Vec < CfgExpr >) , Value (Cfg) , True , False , }
};
}
