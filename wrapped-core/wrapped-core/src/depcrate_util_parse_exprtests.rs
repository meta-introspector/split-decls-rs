// Generated macro for tests (module)
macro_rules! Depcrate_util_parse_exprtests {
() => {
// Module: crate::util::parse_expr
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use syn :: parse_quote ; use super :: * ; macro_rules ! meta { ($ body : expr) => { { let attr : :: syn :: Attribute = :: syn :: parse_quote ! (# [ignore = $ body]) ; attr . meta } } ; } # [test] fn preserve_str () { assert_eq ! (preserve_str_literal (& meta ! ("World")) . unwrap () , parse_quote ! ("World")) ; } # [test] fn preserve_binary_exp () { assert_eq ! (preserve_str_literal (& meta ! ("World" + 5)) . unwrap () , parse_quote ! ("World" + 5)) } # [test] fn parse_ident () { assert_eq ! (parse_str_literal (& meta ! ("world")) . unwrap () , parse_quote ! (world)) } }
};
}
