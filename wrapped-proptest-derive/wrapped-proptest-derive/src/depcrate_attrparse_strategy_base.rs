// Generated macro for parse_strategy_base (function)
macro_rules! Depcrate_attrparse_strategy_base {
() => {
// Module: crate::attr
// Provides: {"parse_strategy_base"}
// Dependencies: {}
# [doc = " Parses an explicit strategy. This is a helper."] # [doc = " Valid forms are:"] # [doc = " + `#[proptest(<meta.name()> = <literal>)]`"] # [doc = " + `#[proptest(<meta.name()> = \"<expr>\")]`"] # [doc = " + `#[proptest(<meta.name()>(\"<expr>\")]`"] # [doc = " + `#[proptest(<meta.name()>(<literal>)]`"] # [doc = " + `#[proptest(<meta.name()>(<ident>)]`"] fn parse_strategy_base (ctx : Ctx , loc : & mut Option < Expr > , meta : & Meta) { error_if_set (ctx , & loc , & meta) ; if let expr @ Some (_) = match normalize_meta (meta . clone ()) { Some (NormMeta :: Word (fun)) => Some (function_call (fun)) , Some (NormMeta :: Lit (lit)) => extract_expr (lit) , _ => None , } { * loc = expr ; } else { error :: strategy_malformed (ctx , meta) } }
};
}
