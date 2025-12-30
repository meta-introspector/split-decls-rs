// Generated macro for Fix (enum)
macro_rules! Depcrate_match_group_validateFix {
() => {
// Module: crate::match_group::validate
// Provides: {"Fix"}
// Dependencies: {}
# [doc = " All possible fixes corrected while validating matched mappings."] # [derive (Debug , PartialEq , Eq , Clone)] pub enum Fix { # [doc = " Removed a mapping that contained a partial destination entirely."] MappingWithPartialDestinationRemoved { # [doc = " The destination ref name that was ignored."] name : BString , # [doc = " The spec that defined the mapping"] spec : RefSpec , } , }
};
}
