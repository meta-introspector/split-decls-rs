// Generated macro for tests (module)
macro_rules! Depcrate_parser_matches_matched_argtests {
() => {
// Module: crate::parser::matches::matched_arg
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_grouped_vals_first () { let mut m = MatchedArg :: new_group () ; m . new_val_group () ; m . new_val_group () ; m . append_val (AnyValue :: new (String :: from ("bbb")) , "bbb" . into ()) ; m . append_val (AnyValue :: new (String :: from ("ccc")) , "ccc" . into ()) ; assert_eq ! (m . first_raw () , Some (& OsString :: from ("bbb"))) ; } }
};
}
