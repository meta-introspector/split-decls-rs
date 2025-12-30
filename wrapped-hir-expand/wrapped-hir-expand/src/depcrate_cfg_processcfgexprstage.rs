// Generated macro for CfgExprStage (enum)
macro_rules! Depcrate_cfg_processCfgExprStage {
() => {
// Module: crate::cfg_process
// Provides: {"CfgExprStage"}
// Dependencies: {}
# [derive (Debug , PartialEq , Eq , Clone , Copy)] enum CfgExprStage { # [doc = " Stripping the CFGExpr part of the attribute"] StrippigCfgExpr , # [doc = " Found the comma after the CFGExpr. Will keep all tokens until the next comma or the end of the attribute"] FoundComma , # [doc = " Everything following the attribute. This could be another attribute or the end of the attribute."] EverythingElse , }
};
}
