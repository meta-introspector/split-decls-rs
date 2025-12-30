// Generated macro for referenced_variables_to_vec (function)
macro_rules! Depcrate_validation_utilsreferenced_variables_to_vec {
() => {
// Module: crate::validation::utils
// Provides: {"referenced_variables_to_vec"}
// Dependencies: {}
fn referenced_variables_to_vec < 'a > (value : & 'a Value , vars : & mut Vec < & 'a str >) { match value { Value :: Variable (name) => { vars . push (name) ; } Value :: List (values) => values . iter () . for_each (| value | referenced_variables_to_vec (value , vars)) , Value :: Object (obj) => obj . values () . for_each (| value | referenced_variables_to_vec (value , vars)) , _ => { } } }
};
}
