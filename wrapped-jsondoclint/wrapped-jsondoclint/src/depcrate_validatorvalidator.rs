// Generated macro for Validator (struct)
macro_rules! Depcrate_validatorValidator {
() => {
// Module: crate::validator
// Provides: {"Validator"}
// Dependencies: {}
# [doc = " The Validator walks over the JSON tree, and ensures it is well formed."] # [doc = " It is made of several parts."] # [doc = ""] # [doc = " - `check_*`: These take a type from [`rustdoc_json_types`], and check that"] # [doc = "              it is well formed. This involves calling `check_*` functions on"] # [doc = "              fields of that item, and `add_*` functions on [`Id`]s."] # [doc = " - `add_*`: These add an [`Id`] to the worklist, after validating it to check if"] # [doc = "            the `Id` is a kind expected in this situation."] # [derive (Debug)] pub struct Validator < 'a > { pub (crate) errs : Vec < Error > , krate : & 'a Crate , krate_json : Value , # [doc = " Worklist of Ids to check."] todo : HashSet < & 'a Id > , # [doc = " Ids that have already been visited, so don't need to be checked again."] seen_ids : HashSet < & 'a Id > , # [doc = " Ids that have already been reported missing."] missing_ids : HashSet < & 'a Id > , }
};
}
