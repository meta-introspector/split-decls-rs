// Generated macro for parse_regex (function)
macro_rules! Depcrate_attrparse_regex {
() => {
// Module: crate::attr
// Provides: {"parse_regex"}
// Dependencies: {}
# [doc = " Parses an explicit value as a strategy."] # [doc = " Valid forms are:"] # [doc = " + `#[proptest(regex = \"<string>\")]`"] # [doc = " + `#[proptest(regex(\"<string>\")]`"] # [doc = " + `#[proptest(regex(<ident>)]`"] fn parse_regex (ctx : Ctx , acc : & mut ParseAcc , meta : & Meta) { error_if_set (ctx , & acc . regex , & meta) ; if let expr @ Some (_) = match normalize_meta (meta . clone ()) { Some (NormMeta :: Word (fun)) => Some (function_call (fun)) , Some (NormMeta :: Lit (lit @ Lit :: Str (_))) => Some (lit_to_expr (lit)) , _ => None , } { acc . regex = expr ; } else { error :: regex_malformed (ctx) } }
};
}
