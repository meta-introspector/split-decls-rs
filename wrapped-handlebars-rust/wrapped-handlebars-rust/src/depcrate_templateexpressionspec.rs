// Generated macro for ExpressionSpec (struct)
macro_rules! Depcrate_templateExpressionSpec {
() => {
// Module: crate::template
// Provides: {"ExpressionSpec"}
// Dependencies: {}
# [non_exhaustive] # [derive (Builder , PartialEq , Eq , Clone , Debug)] pub struct ExpressionSpec { pub name : Parameter , pub params : Vec < Parameter > , pub hash : HashMap < String , Parameter > , # [builder (setter (strip_option) , default)] pub block_param : Option < BlockParam > , pub omit_pre_ws : bool , pub omit_pro_ws : bool , }
};
}
