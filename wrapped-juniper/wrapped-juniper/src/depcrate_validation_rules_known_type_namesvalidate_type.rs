// Generated macro for validate_type (function)
macro_rules! Depcrate_validation_rules_known_type_namesvalidate_type {
() => {
// Module: crate::validation::rules::known_type_names
// Provides: {"validate_type"}
// Dependencies: {}
fn validate_type < S : Debug > (ctx : & mut ValidatorContext < '_ , S > , type_name : & str , location : & SourcePosition ,) { if ctx . schema . type_by_name (type_name) . is_none () { ctx . report_error (& error_message (type_name) , & [* location]) ; } }
};
}
