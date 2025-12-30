// Generated macro for sort_directives (function)
macro_rules! Depcrate_schema_modelsort_directives {
() => {
// Module: crate::schema::model
// Provides: {"sort_directives"}
// Dependencies: {}
# [doc = " Sorts the provided [`DirectiveType`]s by name."] fn sort_directives < S > (directives : & mut [& DirectiveType < S >]) { directives . sort_by (| a , b | a . name . cmp (& b . name)) ; }
};
}
