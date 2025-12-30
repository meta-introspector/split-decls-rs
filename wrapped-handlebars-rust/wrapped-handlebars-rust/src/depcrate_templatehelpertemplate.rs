// Generated macro for HelperTemplate (struct)
macro_rules! Depcrate_templateHelperTemplate {
() => {
// Module: crate::template
// Provides: {"HelperTemplate"}
// Dependencies: {}
# [non_exhaustive] # [derive (Builder , PartialEq , Eq , Clone , Debug)] pub struct HelperTemplate { pub name : Parameter , pub params : Vec < Parameter > , pub hash : HashMap < String , Parameter > , # [builder (setter (strip_option) , default)] pub block_param : Option < BlockParam > , # [builder (setter (strip_option) , default)] pub template : Option < Template > , # [builder (setter (strip_option) , default)] pub inverse : Option < Template > , pub block : bool , pub chain : bool , pub (crate) indent_before_write : bool , }
};
}
