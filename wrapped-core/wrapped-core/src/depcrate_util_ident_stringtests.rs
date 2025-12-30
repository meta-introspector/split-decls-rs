// Generated macro for tests (module)
macro_rules! Depcrate_util_ident_stringtests {
() => {
// Module: crate::util::ident_string
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use syn :: parse_quote ; use super :: IdentString ; # [test] fn convert () { let i_str = IdentString :: new (parse_quote ! (t)) ; assert_eq ! (i_str . as_str () , "t") ; } # [test] fn map_transform () { let i = IdentString :: new (parse_quote ! (my)) ; let after = i . map (| v | format ! ("var_{}" , v)) ; assert_eq ! (after , "var_my") ; assert_eq ! (after , String :: from ("var_my")) ; } }
};
}
