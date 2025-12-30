// Generated macro for validate_type (function)
macro_rules! Depcrate_validation_rules_known_type_namesvalidate_type {
() => {
// Module: crate::validation::rules::known_type_names
// Provides: {"validate_type"}
// Dependencies: {}
fn validate_type (ctx : & mut VisitorContext < '_ > , type_name : & str , pos : Pos) { if ! ctx . registry . types . contains_key (type_name) { ctx . report_error (vec ! [pos] , format ! (r#"Unknown type "{}""# , type_name)) ; } }
};
}
