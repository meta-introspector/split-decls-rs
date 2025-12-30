// Generated macro for tests (module)
macro_rules! Depcrate_util_preserved_str_exprtests {
() => {
// Module: crate::util::preserved_str_expr
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use syn :: { parse_quote , Meta , MetaNameValue } ; # [test] fn preserved_str_expr_from_meta () { let name_value : MetaNameValue = parse_quote ! (test = "Hello, world!") ; let preserved = PreservedStrExpr :: from_meta (& Meta :: NameValue (name_value)) . unwrap () ; assert_eq ! (preserved . 0 , parse_quote ! ("Hello, world!")) ; } }
};
}
