// Generated macro for RuleError (struct)
macro_rules! Depcrate_validation_contextRuleError {
() => {
// Module: crate::validation::context
// Provides: {"RuleError"}
// Dependencies: {}
# [doc = " Query validation error"] # [derive (Clone , Debug , Display , Eq , Error , Ord , PartialEq , PartialOrd)] # [display ("{message}. At {}" , locations . iter () . format (", "))] pub struct RuleError { locations : Vec < SourcePosition > , message : String , }
};
}
