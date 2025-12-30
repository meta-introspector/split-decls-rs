// Generated macro for is_option_or_result (function)
macro_rules! Depcrate_unwrap_in_resultis_option_or_result {
() => {
// Module: crate::unwrap_in_result
// Provides: {"is_option_or_result"}
// Dependencies: {}
fn is_option_or_result (cx : & LateContext < '_ > , ty : Ty < '_ >) -> Option < OptionOrResult > { match ty . ty_adt_def () . and_then (| def | cx . tcx . get_diagnostic_name (def . did ())) { Some (sym :: Option) => Some (OptionOrResult :: Option) , Some (sym :: Result) => Some (OptionOrResult :: Result) , _ => None , } }
};
}
