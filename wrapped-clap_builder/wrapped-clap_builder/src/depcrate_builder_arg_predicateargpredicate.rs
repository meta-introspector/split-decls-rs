// Generated macro for ArgPredicate (enum)
macro_rules! Depcrate_builder_arg_predicateArgPredicate {
() => {
// Module: crate::builder::arg_predicate
// Provides: {"ArgPredicate"}
// Dependencies: {}
# [doc = " Operations to perform on argument values"] # [doc = ""] # [doc = " These do not apply to [`ValueSource::DefaultValue`][crate::parser::ValueSource::DefaultValue]"] # [derive (Clone , Debug , PartialEq , Eq)] # [cfg_attr (feature = "unstable-v5" , non_exhaustive)] pub enum ArgPredicate { # [doc = " Is the argument present?"] IsPresent , # [doc = " Does the argument match the specified value?"] Equals (OsStr) , }
};
}
