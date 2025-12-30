// Generated macro for parse_filter (function)
macro_rules! Depcrate_attrparse_filter {
() => {
// Module: crate::attr
// Provides: {"parse_filter"}
// Dependencies: {}
# [doc = " Parses an explicit value as a strategy."] # [doc = " Valid forms are:"] # [doc = " + `#[proptest(filter(<ident>))]`"] # [doc = " + `#[proptest(filter = \"<expr>\")]`"] # [doc = " + `#[proptest(filter(\"<expr>\")]`"] fn parse_filter (ctx : Ctx , acc : & mut ParseAcc , meta : & Meta) { if let Some (filter) = match normalize_meta (meta . clone ()) { Some (NormMeta :: Lit (Lit :: Str (lit))) => lit . parse () . ok () , Some (NormMeta :: Word (ident)) => Some (parse_quote ! (# ident)) , _ => None , } { acc . filter . push (filter) ; } else { error :: filter_malformed (ctx , meta) } }
};
}
