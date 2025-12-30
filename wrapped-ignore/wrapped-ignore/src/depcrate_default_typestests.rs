// Generated macro for tests (module)
macro_rules! Depcrate_default_typestests {
() => {
// Module: crate::default_types
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: DEFAULT_TYPES ; # [test] fn default_types_are_sorted () { let mut names = DEFAULT_TYPES . iter () . map (| (aliases , _) | aliases [0]) ; let Some (mut previous_name) = names . next () else { return ; } ; for name in names { assert ! (name > previous_name , r#""{}" should be sorted before "{}" in `DEFAULT_TYPES`"# , name , previous_name) ; previous_name = name ; } } }
};
}
